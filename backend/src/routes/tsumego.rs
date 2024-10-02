use actix_web::{
    get,
    web,
    HttpResponse,
    Responder,
};

use crate::{
    model::Tsumego,
    state::State,
};

/// Declares routes for fetching Tsumego data.
pub fn declare_routes(conf: &mut web::ServiceConfig) {
    conf.service(get_tsumego)
        .service(all_tsumego);
}

#[get("/api/problem/{id}")]
async fn get_tsumego(state: State, id: web::Path<i64>) -> impl Responder {
    let tsumego = Tsumego::get_by_id(&state, *id)
        .await;
    
    match tsumego {
        Ok(Some(tsumego)) => HttpResponse::Ok().json(tsumego),
        _ => HttpResponse::NotFound().finish(),
    }
}

#[get("/api/all_problems")]
async fn all_tsumego(state: State) -> impl Responder {
    let problems = Tsumego::get_all(&state)
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
