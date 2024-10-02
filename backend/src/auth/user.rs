use crate::{model::User, result::Result, state::State};

impl User {
    pub async fn check_password(&self, state: &State, given_password: &str) -> Result<bool> {
        let password_hash = sqlx::query_scalar!("SELECT password_hash FROM users WHERE id = ?", self.id)
            .fetch_one(&state.db)
            .await?;
        
        super::hashing::check_password(password_hash.as_str(), given_password)
    }
}
