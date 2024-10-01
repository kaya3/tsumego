use actix_web::{
    get,
    middleware::Logger,
    App,
    HttpServer,
    Responder,
};

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    
    let log_env = env_logger::Env::default()
        .filter("LOG")
        .write_style("LOG_STYLE");
    env_logger::Builder::from_env(log_env)
        .format_module_path(false)
        .init();
    
    HttpServer::new(|| App::new()
        .service(index)
        .wrap(Logger::default())
    )
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
