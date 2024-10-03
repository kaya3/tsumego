use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    http::header,
    middleware::Next,
    Error,
};

use crate::{
    result::{AppError, Result},
    state::State,
};

/// A simple middleware to protect against cross-site requests. The protection
/// is based on inspecting the Referer and Sec-Fetch-Site request headers; this
/// isn't always as good as token-based CSRF protection, but
/// 
/// - This application doesn't have any risk of open redirect vulnerabilities.
/// - Almost every browser does send a Referer header for same-origin POST and
///   DELETE requests.
/// - This application will only be accessed via HTTPS, so the Referer header
///   cannot be removed by a proxy.
/// 
/// Additional protection is also given by the Same-Site attribute for the
/// session token cookie.
pub async fn csrf_middleware(
    request: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // "Safe" methods are GET, HEAD, OPTIONS and TRACE. Other methods are
    // allowed to change application state, so we want to ensure the request
    // isn't cross-site.
    if !request.method().is_safe() && is_bad_request(&request) {
        return Err(AppError::BAD_REQUEST.into());
    }
    
    // Otherwise, forward to the wrapped handler
    next.call(request).await
}

fn is_bad_request(request: &ServiceRequest) -> bool {
    let state: State = request
        .app_data::<State>()
        .expect("State should be available from app data")
        .clone();
    
    // Referer must exist and match this origin
    let expected_origin = state.cfg.base_url.as_ref();
    let referrer = request.headers().get(header::REFERER);
    
    if !matches!(referrer, Some(r) if r.to_str().is_ok_and(|r| r.starts_with(expected_origin))) {
        log::info!("Possible CSRF attack: Referer = {referrer:?}");
        return true;
    }
    
    // If Sec-Fetch-Site exists, it must not be 'cross-site'
    let sec_fetch_site = request.headers().get("Sec-Fetch-Site");
    
    if matches!(sec_fetch_site, Some(r) if r.to_str().is_ok_and(|r| r == "cross-site")) {
        log::info!("Possible CSRF attack: Sec-Fetch-Site = {sec_fetch_site:?}");
        return true;
    }
    
    false
}
