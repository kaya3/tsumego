use actix_web::{
    get, post, web::{ServiceConfig, Json}, HttpRequest, HttpResponse, Responder
};
use serde_json::json;

use crate::{
    auth::{AuthTokenAction, MaybeAuth}, model::{Session, User}, result::{AppError, Result}, state::State
};

/// Declares routes for login/logout and other authentication actions.
pub fn declare_routes(conf: &mut ServiceConfig) {
    conf.service(login)
        .service(logout)
        .service(who_am_i);
}

#[derive(serde::Deserialize)]
struct LoginForm {email: String, password: String}

#[post("/api/login")]
async fn login(state: State, request: HttpRequest, form: Json<LoginForm>) -> Result<impl Responder> {
    let user = User::get_by_email(&state, &form.email)
        .await?;
    
    match user {
        Some(user) if user.check_password(&state, &form.password).await? => {
            log::info!("Successful login for user #{} <{}>", user.id, user.email);
            
            let token = Session::begin_for_user(&state, user.id)
                .await?;
            
            AuthTokenAction::Issue(token)
                .insert_into_request(&request);
            
            Ok(HttpResponse::Ok().json(json!(user)))
        },
        user => {
            let reason = if user.is_some() {"invalid password"} else {"no such user"};
            log::info!("Failed login for <{}>: {reason}", form.email);
            
            // Same response for "no such user" as for "invalid password", to
            // avoid leaking information about which email addresses have
            // registered accounts
            Err(AppError::UNAUTHORIZED)
        },
    }
}

#[post("/api/logout")]
async fn logout(state: State, request: HttpRequest, auth: MaybeAuth) -> Result<impl Responder> {
    if let MaybeAuth::Authenticated {user, session_id} = auth {
        log::info!("Successful logout for user #{} <{}>", user.id, user.email);
        
        Session::revoke_by_id(&state, session_id).await?;
        AuthTokenAction::Revoke.insert_into_request(&request);
    };
    
    Ok(HttpResponse::Ok())
}

#[get("/api/who_am_i")]
async fn who_am_i(auth: MaybeAuth) -> impl Responder {
    HttpResponse::Ok().json(auth.user())
}
