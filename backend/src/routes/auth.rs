use actix_web::{
    get,
    post,
    web::{Json, Query, ServiceConfig},
    HttpRequest,
    HttpResponse,
    Responder,
};

use crate::{
    auth::{AuthTokenAction, MaybeAuth},
    model::{Session, User, UserDetails},
    result::{AppError, Result},
    state::State,
};

/// Declares routes for login/logout and other authentication actions.
pub fn declare_routes(conf: &mut ServiceConfig) {
    conf.service(register_account)
        .service(verify_account)
        .service(login)
        .service(logout)
        .service(who_am_i);
}

#[derive(serde::Deserialize)]
struct RegisterForm {
    email: String,
    #[serde(rename = "displayName")]
    display_name: String,
    password: String,
}

#[post("/api/register")]
async fn register_account(state: State, form: Json<RegisterForm>) -> Result<impl Responder> {
    let form = form.into_inner();
    
    let outcome = User::register(&state, &form.email, &form.display_name, &form.password).await?;
    Ok(HttpResponse::Ok().json(outcome))
}

#[derive(serde::Deserialize)]
struct VerifyForm {
    id: i64,
    code: String,
}

#[get("/verify_account")]
async fn verify_account(state: State, query: Query<VerifyForm>) -> Result<impl Responder> {
    let query = query.into_inner();
    User::verify_account(&state, query.id, &query.code).await?;
    
    // Construct a response with no headers suggesting this is a static file
    let response_body = std::fs::read_to_string("templates/account_verified.html")?;
    let response = HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body);
    
    Ok(response)
}

#[derive(serde::Deserialize)]
struct LoginForm {
    email: String,
    password: String,
}

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
            
            let user_details = UserDetails::get_for_user(&state, user).await?;
            
            Ok(HttpResponse::Ok().json(user_details))
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
async fn who_am_i(state: State, auth: MaybeAuth) -> Result<impl Responder> {
    let user_details = match auth.user() {
        Some(user) => {
            let details = UserDetails::get_for_user(&state, user).await?;
            Some(details)
        },
        None => None,
    };
    
    Ok(HttpResponse::Ok().json(user_details))
}
