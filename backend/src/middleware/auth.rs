use actix_web::{
    body::MessageBody,
    cookie::{time::Duration, Cookie, SameSite},
    dev::{ServiceRequest, ServiceResponse},
    http::header::{HeaderName, HeaderValue},
    middleware::Next,
    Error,
};

use crate::{
    auth::{AuthTokenAction, MaybeAuth},
    result::Result,
    state::State,
};

pub async fn auth_middleware(
    request: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let state: State = request
        .app_data::<State>()
        .expect("State should be available from app data")
        .clone();
    
    // Authenticate by the cookie, if there is one
    let (auth, token_action) = match request.cookie(&state.cfg.session_token_cookie_name) {
        Some(cookie) => MaybeAuth::authenticate_by_session_token(&state, cookie.value()).await?,
        None => (MaybeAuth::Unauthenticated, AuthTokenAction::DoNothing),
    };
    
    // Debug information
    if let Some(u) = auth.clone().user() {
        log::info!("Request made by user #{:?}", u.id);
    } else {
        log::info!("Request not made by any user");
    }
    
    auth.insert_into_request(&request);
    
    // Call the wrapped handler
    let mut response = next.call(request).await?;
    
    // Tell the client not to cache the session token
    // https://cheatsheetseries.owasp.org/cheatsheets/Session_Management_Cheat_Sheet.html#web-content-caching
    response.headers_mut().append(
        HeaderName::from_static("cache-control"),
        HeaderValue::from_static("no-cache=\"Set-Cookie, Set-Cookie2\""),
    );
    
    // Issue or revoke the cookie, if necessary. If an action is inserted into
    // the request object, perform that action; otherwise perform the action
    // indicated by the earlier call to `authenticate_by_session_token`.
    let cookie_name = &state.cfg.session_token_cookie_name;
    match AuthTokenAction::take_from_request(response.request(), token_action) {
        AuthTokenAction::Issue(token) => {
            // Issue a cookie with the appropriate attributes
            // https://cheatsheetseries.owasp.org/cheatsheets/Session_Management_Cheat_Sheet.html#cookies
            let mut cookie = Cookie::new(cookie_name.clone(), token);
            
            // HTTP-only cookies are not visible to client-side JavaScript
            cookie.set_http_only(true);
            // Only send this cookie over HTTPS connections
            cookie.set_secure(true);
            // The client should only send this cookie when making requests
            // from the same site
            cookie.set_same_site(SameSite::Strict);
            
            let duration_days = state.cfg.session_duration_days;
            cookie.set_max_age(Duration::days(duration_days));
            
            response.response_mut().add_cookie(&cookie)?;
        },
        AuthTokenAction::Revoke => {
            // Revoke cookie by setting new empty cookie of the same name
            let cookie = Cookie::new(cookie_name.clone(), "");
            response.response_mut().add_removal_cookie(&cookie)?;
        },
        AuthTokenAction::DoNothing => {},
    }
    
    Ok(response)
}
