use actix_web::{
    get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder
};

mod model;
mod state;

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/problem/{id}")]
async fn get_problem(state: state::State, id: web::Path<i64>) -> impl Responder {
    let tsumego = model::tsumego::Tsumego::get_by_id(&state, *id)
        .await;
    
    match tsumego {
        Ok(Some(tsumego)) => HttpResponse::Ok().json(tsumego),
        _ => HttpResponse::NotFound().finish(),
    }
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
    
    HttpServer::new(move || App::new()
        .app_data(state.clone())
        .service(index)
        .service(get_problem)
        .wrap(Logger::default())
    )
        .bind((host_addr, host_port))?
        .run()
        .await
}
