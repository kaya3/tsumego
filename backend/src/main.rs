use actix_web::{
    middleware::Logger,
    App,
    HttpServer,
};

mod auth;
mod middleware;
mod model;
mod periodic_jobs;
mod result;
mod routes;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv()
        .expect("Failed to load environment variables from '.env'");
    
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
    
    // Run periodic jobs
    periodic_jobs::start(state.clone());
    
    // Set up and start the HTTP server
    HttpServer::new(move || App::new()
        .app_data(state.clone())
        .configure(routes::declare_routes)
        .wrap(actix_web::middleware::from_fn(middleware::auth_middleware))
        // CSRF protection is the outer-most wrapper, so it applies before
        // we authenticate the user
        .wrap(actix_web::middleware::from_fn(middleware::csrf_middleware))
        .wrap(Logger::default())
    )
        .bind((host_addr, host_port))?
        .run()
        .await
}
