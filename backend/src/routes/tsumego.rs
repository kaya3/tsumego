use actix_web::{
    get,
    web::{Path, ServiceConfig},
    HttpResponse,
    Responder,
};
use serde_json::json;

use crate::{
    model::Tsumego,
    result::{OrAppError, Result},
    state::State,
};

/// Declares routes for fetching Tsumego data.
pub fn declare_routes(conf: &mut ServiceConfig) {
    conf.service(get_tsumego)
        .service(all_tsumego);
}

#[get("/api/problem/{id}")]
async fn get_tsumego(state: State, id: Path<i64>) -> Result<impl Responder> {
    let tsumego = Tsumego::get_by_id(&state, *id)
        .await?
        .or_404_not_found()?;
    
    Ok(HttpResponse::Ok().json(tsumego))
}

#[get("/api/all_problems")]
async fn all_tsumego(state: State) -> Result<impl Responder> {
    let problems = Tsumego::get_all(&state)
        .await?;
    
    Ok(HttpResponse::Ok().json(json!({
        "problems": problems,
    })))
}
