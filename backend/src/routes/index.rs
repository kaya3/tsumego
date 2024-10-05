use actix_files::Files;
use actix_web::web::ServiceConfig;

/// Declares routes for serving static content.
pub fn declare_routes(conf: &mut ServiceConfig) {
    let files_service = Files::new("/", "./static")
        .index_file("index.html")
        // `index.html` is served on the `/` route; do not give it a second
        // route.
        .path_filter(|path, _| !path.starts_with("api/") && path.as_os_str() != "index.html");
    
    conf.service(files_service);
}
