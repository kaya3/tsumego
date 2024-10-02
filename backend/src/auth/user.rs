use actix_web::HttpMessage;

use crate::{model::{Session, User}, result::Result, state::State};
use super::{hashing, AuthTokenAction};

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

impl User {
    pub async fn check_password(&self, state: &State, given_password: &str) -> Result<bool> {
        let password_hash = sqlx::query_scalar!("SELECT password_hash FROM users WHERE id = ?", self.id)
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
    
    pub fn insert_into_request(self, request: &impl HttpMessage) {
        request.extensions_mut()
            .insert(self);
    }
    
    pub async fn authenticate_by_session_token(state: &State, token: &str) -> Result<(Self, AuthTokenAction)> {
        #[derive(sqlx::FromRow)]
        struct SessionRecord {
            id: i64,
            user_id: i64,
            days_left: Option<f64>,
        }
        
        let hash = hashing::token_hash(token);
        // This is giving a compilation error with SQLx 0.8.2 but not 0.7.3
        // TODO: report an issue
        let maybe_session = sqlx::query_as!(SessionRecord, "SELECT id, user_id, (julianday(expires) - julianday('now')) AS days_left FROM sessions WHERE token_hash = ? LIMIT 1", hash)
            .fetch_optional(&state.db)
            .await?;
        
        let Some(session) = maybe_session else {
            return Ok((Self::Unauthenticated, AuthTokenAction::DoNothing));
        };
        
        // For some reason, SQLx thinks this is nullable
        let days_left = session.days_left.expect("session expiry should be non-null");
        
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
