mod auth;
mod index;
mod srs;
mod tsumego;

/// Declares all routes for the application.
pub fn declare_routes(conf: &mut actix_web::web::ServiceConfig) {
    auth::declare_routes(conf);
    index::declare_routes(conf);
    srs::declare_routes(conf);
    tsumego::declare_routes(conf);
}
