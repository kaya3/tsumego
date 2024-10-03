use actix_files::NamedFile;
use actix_web::{get, web, Responder};

use crate::result::Result;

/// Declares routes for serving static content.
pub fn declare_routes(conf: &mut web::ServiceConfig) {
    conf.service(index)
        .service(app_js)
        .service(style_css);
}

#[get("/")]
async fn index() -> Result<impl Responder> {
    Ok(NamedFile::open_async("static/index.html").await?)
}

#[get("/app.js")]
async fn app_js() -> Result<impl Responder> {
    Ok(NamedFile::open_async("static/app.js").await?)
}

#[get("/style.css")]
async fn style_css() -> Result<impl Responder> {
    Ok(NamedFile::open_async("static/style.css").await?)
}
