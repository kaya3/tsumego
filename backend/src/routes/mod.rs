use actix_web::{
    web::ServiceConfig,
    FromRequest,
};

mod auth;
mod index;
mod srs;
mod tsumego;

pub use auth::confirmation_link;

/// Declares all routes for the application.
pub fn declare_routes(conf: &mut ServiceConfig) {
    auth::declare_routes(conf);
    srs::declare_routes(conf);
    tsumego::declare_routes(conf);
    
    // Must declare static files last, since this service returns 404 errors
    // for files which don't exist. The other services must take priority to
    // avoid being handled by the static file server.
    index::declare_routes(conf);
}

impl FromRequest for crate::model::User {
    type Error = crate::result::AppError;
    type Future = std::future::Ready<crate::result::Result<crate::model::User>>;

    fn from_request(req: &actix_web::HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let auth = authlogic::require_user_from_request::<crate::state::State>(req);
        std::future::ready(auth)
    }
}