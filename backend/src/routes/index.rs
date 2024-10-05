use actix_files::{Files, NamedFile};
use actix_web::{get, web::ServiceConfig, Responder};

use crate::result::Result;

/// Declares routes for serving static content.
pub fn declare_routes(conf: &mut ServiceConfig) {
    let files_service = Files::new("/", "./static")
        .index_file("index.html")
        // `index.html` is served on the `/` route; do not give it a second
        // route.
        .path_filter(|path, _| path.as_os_str() != "index.html");
    
    conf.service(files_service)
        .service(index);
}

#[get("/")]
async fn index() -> Result<impl Responder> {
    Ok(NamedFile::open_async("static/index.html").await?)
}
