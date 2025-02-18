use crate::{
    model::{time, User},
    result::Result,
    state::{InnerState, State},
};

use authlogic::{
    mail,
    AppDb,
    PasswordHash,
    Secret,
    SessionData,
    UserData,
    UserState,
};

struct UserRecord {
    id: i64,
    email: String,
    display_name: String,
    is_admin: bool,
    password_hash: PasswordHash,
    require_email_verification: bool,
    require_password_change: bool,
}

impl From<UserRecord> for UserData<State> {
    fn from(u: UserRecord) -> Self {
        Self {
            user: User {
                id: u.id,
                email: u.email,
                display_name: u.display_name,
                is_admin: u.is_admin,
            },
            password_hash: u.password_hash,
            state: UserState {
                is_suspended: false,
                require_email_verification: u.require_email_verification,
                require_password_change: u.require_password_change,
            },
        }
    }
}

impl AppDb for State {
    async fn get_user_data_by_id(&self, user_id: i64) -> Result<Option<UserData<State>>> {
        let user = sqlx::query_as!(
            UserRecord,
            r#"SELECT id, email, display_name, is_admin,
                    password_hash "password_hash: PasswordHash",
                    require_email_verification, require_password_change
                FROM users
                WHERE id = ?"#,
            user_id,
        )
            .fetch_optional(&self.db)
            .await?;
        
        Ok(user.map(UserData::from))
    }

    async fn get_user_data_by_identifier(&self, user_identifier: &str) -> Result<Option<UserData<State>>> {
        // The `users.email` column is declared with `NOCASE`, so there is no
        // need to normalise before querying
        let user = sqlx::query_as!(
            UserRecord,
            r#"SELECT id, email, display_name, is_admin,
                    password_hash "password_hash: PasswordHash",
                    require_email_verification, require_password_change
                FROM users
                WHERE email = ?"#,
            user_identifier,
        )
            .fetch_optional(&self.db)
            .await?;
        
        Ok(user.map(UserData::from))
    }
    
    async fn insert_user(&self, data: &UserData<Self>) -> Result<i64> {
        let id = sqlx::query_scalar!(
            "INSERT INTO users
                (email, display_name, is_admin, password_hash, require_password_change, require_email_verification)
                VALUES (?, ?, ?, ?, ?, ?)
                RETURNING id",
            data.user.email,
            data.user.display_name,
            data.user.is_admin,
            data.password_hash,
            data.state.require_password_change,
            data.state.require_email_verification,
        )
            .fetch_one(&self.db)
            .await?;

        Ok(id)
    }

    async fn verify_user(&self, user: &User) -> Result<()> {
        sqlx::query!(
            "UPDATE users SET require_email_verification = 0
                WHERE id = ?",
            user.id,
        )
            .execute(&self.db)
            .await?;

        Ok(())
    }

    async fn update_password(&self, user: &User, password_hash: PasswordHash, then_require_change: bool) -> Result<()> {
        sqlx::query!(
            "UPDATE users SET password_hash = ?, require_password_change = ?
                WHERE id = ?",
            password_hash,
            then_require_change,
            user.id,
        )
            .execute(&self.db)
            .await?;

        Ok(())
    }

    async fn delete_user(&self, user_id: i64) -> Result<()> {
        sqlx::query!("DELETE FROM users WHERE id = ?", user_id)
            .execute(&self.db)
            .await?;
        
        Ok(())
    }
    
    async fn get_session_by_id(&self, session_id: i64) -> Result<Option<SessionData<State>>> {
        let session = sqlx::query!(
            r#"SELECT users.id, users.email, users.display_name, users.is_admin,
                    users.require_email_verification, users.require_password_change,
                    sessions.token_hash "token_hash: Secret",
                    sessions.expires
                FROM sessions INNER JOIN users ON sessions.user_id = users.id
                WHERE sessions.id = ?"#,
            session_id,
        )
            .fetch_optional(&self.db)
            .await?;

        Ok(session.map(|s| SessionData {
            user: User {
                id: s.id,
                email: s.email,
                display_name: s.display_name,
                is_admin: s.is_admin,
            },
            user_state: UserState {
                is_suspended: false,
                require_email_verification: s.require_email_verification,
                require_password_change: s.require_password_change,
            },
            token_hash: s.token_hash,
            expires: s.expires,
        }))
    }

    async fn insert_session(&self, user: &User, token_hash: Secret, expires: time::DateTime) -> Result<i64> {
        let id = sqlx::query_scalar!(
            "INSERT INTO sessions
                (user_id, token_hash, expires)
                VALUES (?, ?, ?)
                RETURNING id",
            user.id,
            token_hash,
            expires,
        )
            .fetch_one(&self.db)
            .await?;

        Ok(id)
    }

    async fn update_session_by_id(&self, session_id: i64, new_token_hash: Secret, expires: time::DateTime) -> Result<()> {
        sqlx::query!(
            "UPDATE sessions SET token_hash = ?, expires = ? WHERE id = ?",
            new_token_hash,
            expires,
            session_id,
        )
            .execute(&self.db)
            .await?;
        
        Ok(())
    }

    async fn delete_session_by_id(&self, session_id: i64) -> Result<()> {
        sqlx::query!("DELETE FROM sessions WHERE id = ?", session_id)
            .execute(&self.db)
            .await?;
        
        Ok(())
    }
    
    async fn get_challenge_by_id(&self, challenge_id: i64) -> Result<Option<mail::ChallengeData<State>>> {
        let record = sqlx::query!(
            r#"SELECT users.id AS user_id, users.email, users.display_name, users.is_admin,
                    challenges.challenge,
                    challenges.code_hash "code_hash: Secret",
                    challenges.expires
                FROM users INNER JOIN challenges ON users.id = challenges.user_id
                WHERE challenges.id = ?"#,
            challenge_id,
        )
            .fetch_optional(&self.db)
            .await?;

        Ok(record.map(|r| mail::ChallengeData {
            user: User {
                id: r.user_id,
                email: r.email,
                display_name: r.display_name,
                is_admin: r.is_admin,
            },
            challenge: r.challenge,
            code_hash: r.code_hash,
            expires: r.expires,
        }))
    }

    async fn insert_challenge(&self, user: &User, challenge: &str, code_hash: Secret, expires: time::DateTime) -> Result<i64> {
        let challenge_id = sqlx::query_scalar!(
            "INSERT INTO challenges (user_id, challenge, code_hash, expires)
                VALUES (?, ?, ?, ?)
                RETURNING id",
            user.id,
            challenge,
            code_hash,
            expires,
        )
            .fetch_one(&self.db)
            .await?;

        Ok(challenge_id)
    }

    async fn delete_challenge_by_id(&self, challenge_id: i64) -> Result<()> {
        sqlx::query!("DELETE FROM challenges WHERE id = ?", challenge_id)
            .execute(&self.db)
            .await?;
        
        Ok(())
    }
}

impl InnerState {
    /// Deletes expired rows from the `sessions` table. This function will be
    /// called periodically.
    pub async fn delete_all_expired_sessions(&self) -> Result<()> {
        let now = time::now();
        
        log::info!("Deleting expired sessions");
        sqlx::query!("DELETE FROM sessions WHERE expires <= ?", now)
            .execute(&self.db)
            .await?;
        
        Ok(())
    }
    
    /// Deletes expired rows from the `challenges` table. This function will be
    /// called periodically.
    pub async fn delete_expired_challenges(&self) -> Result<()> {
        let now = time::now();
        
        log::info!("Deleting expired challenges");
        sqlx::query!("DELETE FROM challenges WHERE expires <= ?", now)
            .execute(&self.db)
            .await?;
        
        Ok(())
    }
}
