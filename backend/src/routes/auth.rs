use actix_web::{
    get, post, web, HttpResponse, Responder
};
use serde_json::{json, Value};

use crate::{
    auth::MaybeAuth, model::User, result::{AppError, Result}, state::State
};

/// Declares routes for login/logout and other authentication actions.
pub fn declare_routes(conf: &mut web::ServiceConfig) {
    conf.service(login)
        .service(who_am_i);
}

#[derive(serde::Deserialize)]
struct LoginForm {email: String, password: String}

#[post("/api/login")]
async fn login(state: State, form: web::Form<LoginForm>) -> Result<impl Responder> {
    let user = User::get_by_email(&state, &form.email)
        .await?;
    
    match user {
        Some(user) if user.check_password(&state, &form.password).await? => {
            log::info!("Successful login for user #{} <{}>", user.id, user.email);
            
            let token = user.new_session_token(&state)
                .await?;
            
            // TODO: set cookie
            
            // Empty response body, as the frontend only needs to check the
            // response's status code
            Ok(HttpResponse::Ok())
        },
        user => {
            let reason = if user.is_some() {"invalid password"} else {"no such user"};
            log::info!("Failed login for <{}>: {reason}", form.email);
            
            // Same response for "no such user" as for "invalid password", to
            // avoid leaking information about which email addresses have
            // registered accounts
            Err(AppError::Unauthorised)
        },
    }
}

#[get("/api/who_am_i")]
async fn who_am_i(user: MaybeAuth) -> Result<impl Responder> {
    Ok(HttpResponse::Ok().json(user.user()))
}
