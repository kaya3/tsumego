use actix_files::NamedFile;
use actix_web::{get, web, Responder};

/// Declares routes for serving static content.
pub fn declare_routes(conf: &mut web::ServiceConfig) {
    conf.service(index)
        .service(tsumego_js);
}

#[get("/")]
async fn index() -> std::io::Result<impl Responder> {
    NamedFile::open_async("static/index.html").await
}

#[get("/out/tsumego.js")]
async fn tsumego_js() -> std::io::Result<impl Responder> {
    NamedFile::open_async("static/tsumego.js").await
}
