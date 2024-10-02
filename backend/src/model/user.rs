use crate::{
    result::Result, state::State
};

#[derive(sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub display_name: String,
    pub is_admin: bool,
}

impl User {
    pub async fn require_by_id(state: &State, id: i64) -> Result<Self> {
        Self::get_by_id(state, id)
            .await?
            .map_or(Err(sqlx::Error::RowNotFound.into()), Ok)
    }
    
    pub async fn get_by_id(state: &State, id: i64) -> Result<Option<Self>> {
        Ok(sqlx::query_as!(Self, "SELECT id, email, display_name, is_admin FROM users WHERE id = ?", id)
            .fetch_optional(&state.db)
            .await?)
    }
    
    pub async fn get_by_email(state: &State, email: &str) -> Result<Option<Self>> {
        // The `users.email` column is declared with `NOCASE`, so there is no
        // need to normalise before querying
        Ok(sqlx::query_as!(Self, "SELECT id, email, display_name, is_admin FROM users WHERE email = ? LIMIT 1", email)
            .fetch_optional(&state.db)
            .await?)
    }
    
    pub async fn get_all(state: &State) -> Result<Vec<Self>> {
        Ok(sqlx::query_as!(Self, "SELECT id, email, display_name, is_admin FROM users ORDER by id")
            .fetch_all(&state.db)
            .await?)
    }
}
