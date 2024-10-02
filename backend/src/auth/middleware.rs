use actix_web::{body::MessageBody, dev::{ServiceRequest, ServiceResponse}, middleware, Error, FromRequest, HttpMessage};

use crate::{result::{AppError, Result}, state::State};

use super::MaybeAuth;

pub async fn auth_middleware(
    req: ServiceRequest,
    next: middleware::Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let state: State = req.app_data::<State>().unwrap().clone();
    
    let auth = match req.cookie(&state.cfg.session_token_cookie_name) {
        Some(cookie) => MaybeAuth::get_by_session_token(&state, cookie.value()).await?,
        None => MaybeAuth::Unauthenticated,
    };
    
    if let MaybeAuth::Authenticated(ref u) = auth {
        log::info!("Request made by user #{:?}", u.id);
    } else {
        log::info!("Request not made by any user");
    }
    
    req.extensions_mut().insert::<MaybeAuth>(auth);
    
    let mut response = next.call(req).await?;
    // TODO: update cookie according to result
    Ok(response)
}

fn maybe_user_from_req(req: &actix_web::HttpRequest) -> MaybeAuth {
    req.extensions()
        .get::<MaybeAuth>()
        .cloned()
        .unwrap_or(MaybeAuth::Unauthenticated)
}

impl FromRequest for MaybeAuth {
    type Error = AppError;
    type Future = std::future::Ready<Result<Self>>;
    
    fn from_request(req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
        let u = maybe_user_from_req(req);
        std::future::ready(Ok(u))
    }
}
