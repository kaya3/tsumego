use actix_files::NamedFile;
use actix_web::{get, web, Responder};

use crate::result::Result;

/// Declares routes for serving static content.
pub fn declare_routes(conf: &mut web::ServiceConfig) {
    conf.service(index)
        .service(tsumego_js);
}

#[get("/")]
async fn index() -> Result<impl Responder> {
    Ok(NamedFile::open_async("static/index.html").await?)
}

#[get("/out/tsumego.js")]
async fn tsumego_js() -> Result<impl Responder> {
    Ok(NamedFile::open_async("static/tsumego.js").await?)
}
