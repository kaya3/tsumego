use crate::{model::Session, result::Result, state::State};

impl Session {
    pub async fn revoke_by_id(state: &State, id: i64) -> Result<()> {
        sqlx::query!("DELETE FROM sessions WHERE id = ?", id)
            .execute(&state.db)
            .await?;
        
        Ok(())
    }
    
    pub async fn revoke_by_token(state: &State, token: &str) -> Result<()> {
        let hash = super::hashing::token_hash(token);
        
        sqlx::query!("DELETE FROM sessions WHERE token_hash = ?", hash)
            .execute(&state.db)
            .await?;
        
        Ok(())
    }
}
