use actix_web::{
    post, web, HttpResponse, Responder
};

use crate::{
    model::{User, Session},
    result::{AppError, Result},
    state::State,
};

/// Declares routes for login/logout and other authentication actions.
pub fn declare_routes(conf: &mut web::ServiceConfig) {
    conf.service(login);
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
            
            let token = Session::new_token_for_user(&state, &user)
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
