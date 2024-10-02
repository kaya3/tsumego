use crate::{
    auth, model::User, result::Result, state::State
};

#[derive(sqlx::FromRow)]
pub struct Session {
    pub id: i64,
    pub user_id: i64,
    pub expires: chrono::NaiveDateTime,
}

impl Session {
    const TOKEN_GENERATOR_MAX_RETRIES: usize = 32;
    
    /// Generates a new session token for the given user, and inserts it into
    /// the database. This should be called on a successful login.
    pub async fn new_token_for_user(state: &State, user: &User) -> Result<String> {
        // Check for duplicate tokens and regenerate them. It is astronomically
        // unlikely that a duplicate will be generated, unless `thread_rng` is
        // defective.
        let mut retries: usize = 0;
        let (token, hash) = loop {
            let (token, hash) = auth::generate_session_token();
            
            let is_duplicate = sqlx::query_scalar!("SELECT COUNT(1) FROM sessions WHERE token_hash = ? LIMIT 1", hash)
                .fetch_one(&state.db)
                .await? > 0;
            
            if !is_duplicate {
                break (token, hash);
            }
            
            retries += 1;
            if retries >= Self::TOKEN_GENERATOR_MAX_RETRIES {
                // If this ever happens, the RNG is catastrophically broken.
                // Perhaps it is returning all zeroes, or something.
                panic!("thread_rng has catastrophically low entropy");
            }
        };
        
        if retries > 0 {
            let unlikeliness = 8 * auth::SESSION_TOKEN_BYTES * retries;
            log::warn!("Session token generated with {retries} retry(s); thread_rng probably has low entropy, as otherwise this event has probability 2^-{unlikeliness}");
        }
        
        sqlx::query!("INSERT INTO sessions (user_id, token_hash, expires) VALUES (?, ?, datetime('now', '+'||?||' days'))", user.id, hash, state.cfg.session_duration_days)
            .execute(&state.db)
            .await?;
        
        Ok(token)
    }
}
