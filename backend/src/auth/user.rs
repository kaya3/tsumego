use actix_web::{
    dev::Payload,
    FromRequest,
    HttpMessage,
    HttpRequest,
};

use crate::{
    auth::{hashing, AuthTokenAction},
    model::{time, Session, User},
    result::{AppError, OrAppError, Result},
    state::State,
};

impl User {
    pub async fn check_password(&self, state: &State, given_password: &str) -> Result<bool> {
        let password_hash = sqlx::query_scalar!(
            "SELECT password_hash FROM users WHERE id = ?",
            self.id,
        )
            .fetch_one(&state.db)
            .await?;
        
        hashing::check_password(password_hash.as_str(), given_password)
    }
}

impl MaybeAuth {
    /// Gets the authenticated user, if there is one.
    pub fn user(self) -> Option<User> {
        match self {
            MaybeAuth::Authenticated {user, ..} => Some(user),
            MaybeAuth::Unauthenticated => None,
        }
    }
    
    /// Registers this authentication state with the request, so that route
    /// handlers can get the current authentication state. This should only
    /// be called by the authentication middleware.
    pub fn insert_into_request(self, request: &impl HttpMessage) {
        request.extensions_mut()
            .insert(self);
    }
    
    /// Gets the authentication state for the request. This can be called from
    /// route handlers.
    pub fn get_from_request(request: &HttpRequest) -> Self {
        request.extensions()
            .get::<Self>()
            .cloned()
            .unwrap_or(Self::Unauthenticated)
    }
    
    /// Determines the authentication state from the user's session token, and
    /// a needed action (if any) to update the client's cookie in case their
    /// token is renewed, or the token is expired or otherwise invalid.
    pub async fn authenticate_by_session_token(state: &State, token: &str) -> Result<(Self, AuthTokenAction)> {
        let hash = hashing::token_hash(token);
        // This is giving a compilation error with SQLx 0.8.2 but not 0.7.3
        // TODO: report an issue
        let maybe_session = sqlx::query_as!(
            Session,
            "SELECT id, user_id, expires
                FROM sessions
                WHERE token_hash = ?
                LIMIT 1",
            hash,
        )
            .fetch_optional(&state.db)
            .await?;
        
        let Some(session) = maybe_session else {
            // The user's cookie doesn't match a valid session - revoke it.
            // They probably have an old cookie, the session expired and was
            // deleted from the database.
            return Ok((Self::Unauthenticated, AuthTokenAction::Revoke));
        };
        
        let days_left = time::delta_days(time::now(), session.expires);
        
        // Check whether to revoke an expired session
        if days_left <= 0.0 {
            Session::revoke_by_id(state, session.id).await?;
            return Ok((Self::Unauthenticated, AuthTokenAction::Revoke));
        }
        
        // Check whether to renew the session
        let renewal_period = (state.cfg.session_duration_days - state.cfg.session_renew_after_days) as f64;
        let token_action = if days_left <= renewal_period {
            log::info!("Renewing session #{} for {} days", session.id, state.cfg.session_duration_days);
            let token = Session::renew(state, session.id).await?;
            AuthTokenAction::Issue(token)
        } else {
            AuthTokenAction::DoNothing
        };
        
        let user = User::require_by_id(state, session.user_id)
            .await?;
        
        let auth = Self::Authenticated {
            user,
            session_id: session.id,
        };
        Ok((auth, token_action))
    }
}

/// Represents either an authenticated user, or that the current user is not
/// authenticated. Equivalent to an `Option` of `(user, session_id)`, but Rust
/// doesn't allow implementing third-party traits like `actix_web::FromRequest`
/// for built-in types like `Option`.
#[derive(Clone)]
pub enum MaybeAuth {
    Authenticated {user: User, session_id: i64},
    Unauthenticated,
}

impl From<MaybeAuth> for Option<User> {
    fn from(auth: MaybeAuth) -> Self {
        auth.user()
    }
}

impl FromRequest for MaybeAuth {
    type Error = AppError;
    type Future = std::future::Ready<Result<Self>>;
    
    fn from_request(request: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let auth = MaybeAuth::get_from_request(request);
        std::future::ready(Ok(auth))
    }
}

impl FromRequest for User {
    type Error = AppError;
    type Future = std::future::Ready<Result<Self>>;
    
    fn from_request(request: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let result = MaybeAuth::get_from_request(request)
            .user()
            .or_401_unauthorised();
        
        std::future::ready(result)
    }
}
