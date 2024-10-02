mod index;
mod tsumego;

/// Declares all routes for the application.
pub fn declare_routes(conf: &mut actix_web::web::ServiceConfig) {
    index::declare_routes(conf);
    tsumego::declare_routes(conf);
}
