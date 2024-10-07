use actix_web::{
    post,
    web::{Json, ServiceConfig},
    HttpResponse,
    Responder,
};

use crate::{
    model::{Grade, User, UserTsumegoStats},
    result::Result,
    state::State,
};

/// Declares routes for fetching Tsumego data.
pub fn declare_routes(conf: &mut ServiceConfig) {
    conf.service(post_review);
}

#[derive(serde::Deserialize)]
struct Review {
    #[serde(rename = "tsumegoID")]
    tsumego_id: i64,
    grade: Grade,
}

#[post("/api/review")]
async fn post_review(state: State, user: User, review: Json<Review>) -> Result<impl Responder> {
    let stats = UserTsumegoStats::update_on_review(&state, user.id, review.tsumego_id, review.grade)
        .await?;
    
    Ok(HttpResponse::Ok().json(stats))
}
