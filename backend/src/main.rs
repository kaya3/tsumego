mod auth;
mod config;
mod middleware;
mod model;
mod periodic_jobs;
mod result;
mod routes;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from `.env` in the project directory
    dotenvy::dotenv()
        .expect("Failed to load environment variables from '.env'");
    
    // Configure logging
    let log_env = env_logger::Env::default()
        .filter("LOG")
        .write_style("LOG_STYLE");
    env_logger::Builder::from_env(log_env)
        .format_module_path(false)
        .init();
    
    // Initialise application state
    let state = state::from_env().await;
    
    // Start periodic jobs on a loop
    periodic_jobs::start(state.clone());
    
    // Set up and start the HTTP server
    let host_addr = state.cfg.host_addr.to_string();
    let host_port = state.cfg.host_port;
    println!("Listening on {host_addr}:{host_port}");
    
    use actix_web::{App, HttpServer, middleware};

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .configure(routes::declare_routes)
            .wrap(middleware::from_fn(authlogic::middleware::<state::State>))
            // CSRF protection applies before we authenticate the user
            .wrap(middleware::from_fn(crate::middleware::csrf_middleware))
            // Logging is the outer-most middleware, so the log can see all
            // requests before other middleware touches them, and all responses
            // after they are finalised
            .wrap(middleware::Logger::default())
    })
        .bind((host_addr, host_port))?
        .run()
        .await
}
