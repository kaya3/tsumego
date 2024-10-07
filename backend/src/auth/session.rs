use crate::{
    auth::hashing,
    model::{time, Session},
    result::Result,
    state::State,
};

impl Session {
    /// Revokes a session by its id, deleting it from the database if it
    /// exists. It is not an error to attempt to revoke a non-existent session.
    pub async fn revoke_by_id(state: &State, id: i64) -> Result<()> {
        sqlx::query!("DELETE FROM sessions WHERE id = ?", id)
            .execute(&state.db)
            .await?;
        
        Ok(())
    }
    
    /// Deletes expired rows from the `sessions` table. This function will be
    /// called periodically.
    pub async fn delete_all_expired(state: &State) -> Result<()> {
        let now = time::now();
        
        log::info!("Deleting expired sessions");
        sqlx::query!("DELETE FROM sessions WHERE expires <= ?", now)
            .execute(&state.db)
            .await?;
        
        Ok(())
    }
    
    /// Generates a new session token for the given user, and inserts the
    /// session into the database. This function should be called on a
    /// successful login.
    pub async fn begin_for_user(state: &State, user_id: i64) -> Result<String> {
        let (token, hash) = generate_new_session_token(state)
            .await?;
        
        let expires = get_expiry_time_from_now(state);
        sqlx::query!(
            "INSERT INTO sessions
                (user_id, token_hash, expires)
                VALUES (?, ?, ?)",
            user_id,
            hash,
            expires,
        )
            .execute(&state.db)
            .await?;
        
        Ok(token)
    }
    
    /// Renews the session with the given id, updates the session in the
    /// database, and returns the new session token. This function should be
    /// called on successful authentication if the session is within the
    /// renewal period.
    pub async fn renew(state: &State, session_id: i64) -> Result<String> {
        let (token, hash) = generate_new_session_token(state)
            .await?;
        
        let expires = get_expiry_time_from_now(state);
        sqlx::query!(
            "UPDATE sessions SET
                token_hash = ?,
                expires = ?
                WHERE id = ?",
            hash,
            expires,
            session_id,
        )
            .execute(&state.db)
            .await?;
        
        Ok(token)
    }
}

/// Helper function which computes the expiry time for a session which begins
/// or is renewed at the current time.
fn get_expiry_time_from_now(state: &State) -> time::DateTime {
    time::add_days(time::now(), state.cfg.session_duration_days as f64)
}

/// Helper function which generates a new random session token, and its hash.
async fn generate_new_session_token(state: &State) -> Result<(String, String)> {
    // Check for duplicate tokens and regenerate them. It is astronomically
    // unlikely that a duplicate will be generated, unless `thread_rng` is
    // defective.
    const MAX_RETRIES: usize = 32;
    
    let mut retries: usize = 0;
    let (token, hash) = loop {
        let (token, hash) = hashing::generate_session_token();
        
        let is_duplicate = sqlx::query_scalar!(
            "SELECT COUNT(1) FROM sessions
                WHERE token_hash = ?
                LIMIT 1",
            hash,
        )
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
    
    Ok((token, hash))
}
