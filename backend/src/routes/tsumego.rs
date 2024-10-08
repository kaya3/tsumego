use actix_web::{
    get,
    web::{Path, Query, ServiceConfig},
    HttpResponse,
    Responder,
};
use serde_json::json;

use crate::{
    model::{Tsumego, User},
    result::{AppError, OrAppError, Result},
    state::State,
};

/// Declares routes for fetching Tsumego data.
pub fn declare_routes(conf: &mut ServiceConfig) {
    conf.service(get_tsumego)
        .service(get_pending)
        .service(get_random_unstudied);
}

#[get("/api/problem/{id}")]
async fn get_tsumego(state: State, id: Path<i64>) -> Result<impl Responder> {
    let tsumego = Tsumego::get_by_id(&state, *id)
        .await?
        .or_404_not_found()?;
    
    Ok(HttpResponse::Ok().json(tsumego))
}

#[get("/api/get_pending")]
async fn get_pending(state: State, user: User) -> Result<impl Responder> {
    let pending = Tsumego::get_pending(&state, user.id)
        .await?;
    
    Ok(HttpResponse::Ok().json(json!({
        "problems": pending,
    })))
}

#[derive(serde::Deserialize)]
struct GetProblemsLimit {
    limit: i64,
}

#[get("/api/get_random_unstudied")]
async fn get_random_unstudied(state: State, user: User, limit: Query<GetProblemsLimit>) -> Result<impl Responder> {
    let limit = limit.into_inner().limit;
    if limit < 1 || limit > state.cfg.max_problems_at_once {
        return Err(AppError::BAD_REQUEST);
    }
    
    let mut problems = Tsumego::get_random_unstudied(&state, user.id, limit)
        .await?;
    
    // If we didn't find any new problems, choose some old ones.
    if problems.is_empty() {
        problems = Tsumego::get_random(&state, limit)
            .await?;
    }
    
    Ok(HttpResponse::Ok().json(json!({
        "problems": problems,
    })))
}
