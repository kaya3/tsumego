use actix_web::{body::MessageBody, dev::{ServiceRequest, ServiceResponse}, middleware, Error, HttpMessage};

use crate::{model::User, result::Result, state::State};

pub async fn auth_middleware(
    req: ServiceRequest,
    next: middleware::Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let state: State = req.app_data::<State>().unwrap().clone();
    
    let user = match req.cookie(&state.cfg.session_token_cookie_name) {
        Some(cookie) => User::get_by_session_token(&state, cookie.value()).await?,
        None => None,
    };
    
    if let Some(ref u) = user {
        log::info!("Request made by user #{:?}", u.id);
    } else {
        log::info!("Request not made by any user");
    }
    
    req.extensions_mut().insert(user);
    
    next.call(req).await
}
