use actix_web::web::ServiceConfig;

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
