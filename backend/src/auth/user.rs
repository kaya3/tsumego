use crate::{model::{Session, User}, result::Result, state::State};
use super::hashing;

/// Represents either an authenticated user, or that the current user is not
/// authenticated. Equivalent to `Option<User>`, but Rust doesn't allow
/// implementing third-party traits like `actix_web::FromRequest` for built-in
/// types like `Option`.
#[derive(Clone)]
pub enum MaybeAuth {
    Authenticated(User),
    Unauthenticated,
}

impl From<MaybeAuth> for Option<User> {
    fn from(user: MaybeAuth) -> Self {
        user.user()
    }
}

impl User {
    pub async fn check_password(&self, state: &State, given_password: &str) -> Result<bool> {
        let password_hash = sqlx::query_scalar!("SELECT password_hash FROM users WHERE id = ?", self.id)
            .fetch_one(&state.db)
            .await?;
        
        hashing::check_password(password_hash.as_str(), given_password)
    }
    
    /// Generates a new session token for the given user, and inserts it into
    /// the database. This should be called on a successful login.
    pub async fn new_session_token(&self, state: &State) -> Result<String> {
        // Check for duplicate tokens and regenerate them. It is astronomically
        // unlikely that a duplicate will be generated, unless `thread_rng` is
        // defective.
        const MAX_RETRIES: usize = 32;
        
        let mut retries: usize = 0;
        let (token, hash) = loop {
            let (token, hash) = hashing::generate_session_token();
            
            let is_duplicate = sqlx::query_scalar!("SELECT COUNT(1) FROM sessions WHERE token_hash = ? LIMIT 1", hash)
                .fetch_one(&state.db)
                .await? > 0;
            
            if !is_duplicate {
                break (token, hash);
            }
            
            retries += 1;
            if retries >= MAX_RETRIES {
                // If this ever happens, the RNG is catastrophically broken.
                // Perhaps it is returning all zeroes, or something.
                panic!("thread_rng has catastrophically low entropy");
            }
        };
        
        if retries > 0 {
            let unlikeliness = 8 * hashing::SESSION_TOKEN_BYTES * retries;
            log::warn!("Session token generated with {retries} retry(s); thread_rng probably has low entropy, as otherwise this event has probability 2^-{unlikeliness}");
        }
        
        sqlx::query!("INSERT INTO sessions (user_id, token_hash, expires) VALUES (?, ?, datetime('now', '+'||?||' days'))", self.id, hash, state.cfg.session_duration_days)
            .execute(&state.db)
            .await?;
        
        Ok(token)
    }
}

impl MaybeAuth {
    /// Gets the authenticated user, if there is one.
    pub fn user(self) -> Option<User> {
        match self {
            MaybeAuth::Authenticated(user) => Some(user),
            MaybeAuth::Unauthenticated => None,
        }
    }
    
    pub async fn get_by_session_token(state: &State, token: &str) -> Result<Self> {
        #[derive(sqlx::FromRow)]
        struct SessionRecord {
            id: i64,
            user_id: i64,
            days_left: Option<f64>,
        }
        
        let hash = hashing::token_hash(token);
        // This is giving a compilation error with SQLx 0.8.2 but not 0.7.3
        // TODO: report an issue
        let Some(session) = sqlx::query_as!(SessionRecord, "SELECT id, user_id, (julianday(expires) - julianday('now')) AS days_left FROM sessions WHERE token_hash = ? LIMIT 1", hash)
            .fetch_optional(&state.db)
            .await?
            else { return Ok(Self::Unauthenticated); };
        
        let days_left = session.days_left.expect("session expiry should be non-null");
        
        if days_left <= 0.0 {
            // Session is expired
            Session::revoke_by_id(state, session.id).await?;
            return Ok(Self::Unauthenticated);
        }
        
        let user = User::require_by_id(state, session.user_id)
            .await?;
        
        Ok(Self::Authenticated(user))
    }
}
