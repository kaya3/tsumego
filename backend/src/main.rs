use actix_web::{
    get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder
};

mod model;
mod state;

#[get("/")]
async fn index() -> impl Responder {
    let Ok(index_html) = std::fs::read_to_string("../frontend/index.html") else {
        return HttpResponse::NotFound().finish();
    };
    
    HttpResponse::Ok()
        .content_type("text/html")
        .body(index_html)
}

#[get("/out/tsumego.js")]
async fn tsumego_js() -> impl Responder {
    let Ok(index_html) = std::fs::read_to_string("../frontend/out/tsumego.js") else {
        return HttpResponse::NotFound().finish();
    };
    
    HttpResponse::Ok()
        .content_type("text/javascript")
        .body(index_html)
}

#[get("/api/problem/{id}")]
async fn get_problem(state: state::State, id: web::Path<i64>) -> impl Responder {
    let tsumego = model::tsumego::Tsumego::get_by_id(&state, *id)
        .await;
    
    match tsumego {
        Ok(Some(tsumego)) => HttpResponse::Ok().json(tsumego),
        _ => HttpResponse::NotFound().finish(),
    }
}

#[get("/api/all_problems")]
async fn get_problems(state: state::State) -> impl Responder {
    let problems = model::tsumego::Tsumego::get_all(&state)
        .await;
    
    match problems {
        Ok(problems) => HttpResponse::Ok().json(serde_json::json!({
            "problems": problems,
        })),
        Err(e) => {
            log::error!("Error loading all problems: {e}");
            HttpResponse::InternalServerError().finish()
        },
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
        .service(tsumego_js)
        .service(get_problem)
        .service(get_problems)
        .wrap(Logger::default())
    )
        .bind((host_addr, host_port))?
        .run()
        .await
}
