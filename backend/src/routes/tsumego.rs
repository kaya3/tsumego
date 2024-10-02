use actix_web::{
    get,
    web,
    HttpResponse,
    Responder,
};

use crate::{
    model::Tsumego,
    result::{AppError, Result},
    state::State,
};

/// Declares routes for fetching Tsumego data.
pub fn declare_routes(conf: &mut web::ServiceConfig) {
    conf.service(get_tsumego)
        .service(all_tsumego);
}

#[get("/api/problem/{id}")]
async fn get_tsumego(state: State, id: web::Path<i64>) -> Result<impl Responder> {
    let tsumego = Tsumego::get_by_id(&state, *id)
        .await?;
    
    match tsumego {
        Some(tsumego) => Ok(HttpResponse::Ok().json(tsumego)),
        _ => Err(AppError::NotFound),
    }
}

#[get("/api/all_problems")]
async fn all_tsumego(state: State) -> Result<impl Responder> {
    let problems = Tsumego::get_all(&state)
        .await?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "problems": problems,
    })))
}
