use actix_web::{
    get,
    middleware::Logger,
    App,
    HttpServer,
    Responder,
};

mod state;

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().expect("Failed to load environment variables from '.env'");
    
    let log_env = env_logger::Env::default()
        .filter("LOG")
        .write_style("LOG_STYLE");
    env_logger::Builder::from_env(log_env)
        .format_module_path(false)
        .init();
    
    let state = state::from_env().await;
    let host_addr = state.cfg.host_addr.to_string();
    let host_port = state.cfg.host_port;
    
    println!("Listening on {host_addr}:{host_port}");
    
    HttpServer::new(|| App::new()
        .service(index)
        .wrap(Logger::default())
    )
        .bind((host_addr, host_port))?
        .run()
        .await
}
