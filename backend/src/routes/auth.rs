use actix_web::{
    get,
    post,
    web::{Json, Query, Redirect, ServiceConfig},
    HttpRequest,
    HttpResponse,
    Responder,
};

use authlogic::Secret;

use crate::{
    auth::MaybeAuth,
    model::{User, UserDetails},
    result::Result,
    state::State,
};

/// Declares routes for login/logout and other authentication actions.
pub fn declare_routes(conf: &mut ServiceConfig) {
    conf.service(register_account)
        .service(verify_challenge)
        .service(login)
        .service(logout)
        .service(who_am_i);
}

#[derive(serde::Deserialize)]
struct RegisterForm {
    email: String,
    #[serde(rename = "displayName")]
    display_name: String,
    password: Secret,
}

#[post("/api/register")]
async fn register_account(state: State, form: Json<RegisterForm>) -> Result<impl Responder> {
    let form = form.into_inner();
    let user = User {
        id: -1,
        email: form.email,
        display_name: form.display_name,
        is_admin: false,
    };
    
    let outcome = crate::auth::register(&state, user, form.password)
        .await?;
    Ok(HttpResponse::Ok().json(outcome))
}

#[derive(serde::Deserialize)]
struct ChallengeForm {
    code: Secret,
}

/// Builds a URL for a link which completes a challenge when visited.
pub fn confirmation_link(state: &State, code: Secret) -> String {
    // This must match the `verify` route URL
    let base_url = &state.cfg.base_url;
    format!("{base_url}verify?code={}", code.expose())
}

#[get("/verify")]
async fn verify_challenge(state: State, request: HttpRequest, query: Query<ChallengeForm>) -> Result<impl Responder> {
    use authlogic::mail::{Challenge, complete_challenge};
    
    let code = query.into_inner().code;
    let (_, challenge) = complete_challenge(&state, code, &request)
        .await?;
    
    let response_body = match challenge {
        Challenge::LogIn |
        Challenge::ResetPassword => {
            let response = Redirect::to("/")
                .temporary()
                .respond_to(&request)
                .map_into_boxed_body();
            return Ok(response);
        },
        Challenge::VerifyNewUser => {
            std::fs::read_to_string("templates/account_verified.html")?
        },
        Challenge::Custom(c) => c.never_happens(),
    };
    
    // Construct a response manually; using `actix_files` would send cache
    // headers, which we don't want
    let response = HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body);
    
    Ok(response)
}

#[derive(serde::Deserialize)]
struct LoginForm {
    email: String,
    password: Secret,
}

#[post("/api/login")]
async fn login(state: State, request: HttpRequest, form: Json<LoginForm>) -> Result<impl Responder> {
    let form = form.into_inner();
    let user = authlogic::login(&state, &form.email, form.password, &request)
        .await?;
    
    let user_details = UserDetails::get_for_user(&state, user)
        .await?;
    Ok(HttpResponse::Ok().json(user_details))
}

#[post("/api/logout")]
async fn logout(state: State, request: HttpRequest, auth: MaybeAuth) -> Result<impl Responder> {
    auth.logout(&state, &request)
        .await?;
    
    Ok(HttpResponse::Ok())
}

#[get("/api/who_am_i")]
async fn who_am_i(state: State, auth: MaybeAuth) -> Result<impl Responder> {
    let user_details = match auth.user() {
        Some(user) => {
            let details = UserDetails::get_for_user(&state, user)
                .await?;
            Some(details)
        },
        None => None,
    };
    
    Ok(HttpResponse::Ok().json(user_details))
}
