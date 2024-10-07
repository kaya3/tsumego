use crate::{auth::hashing::{check_password, generate_password_hash}, model::{time, User}, result::{AppError, OrAppError, Result}, state::State};

use super::{hashing::generate_verification_code, send_confirmation_email};

struct UnverifiedUser {
    verification_id: i64,
    email: String,
    display_name: String,
    password_hash: String,
    code_hash: String,
}

#[derive(serde::Serialize)]
pub struct RegistrationOutcome {
    #[serde(rename = "verificationID")]
    pub verification_id: i64,
    pub error: Option<RegistrationError>,
}

#[derive(serde::Serialize)]
pub enum RegistrationError {
    #[serde(rename = "Email already exists")]
    EmailAlreadyExists,
}

impl RegistrationOutcome {
    const EMAIL_ALREADY_EXISTS: Self = Self {
        verification_id: -1,
        error: Some(RegistrationError::EmailAlreadyExists),
    };
}

impl User {
    pub async fn register(state: &State, email: &str, display_name: &str, password: &str) -> Result<RegistrationOutcome> {
        // Check if this email is already in use
        let email_already_exists = sqlx::query_scalar!("SELECT COUNT(1) FROM (SELECT 1 FROM users WHERE email = ? UNION SELECT 1 FROM user_verification_codes WHERE email = ?)", email, email)
            .fetch_one(&state.db)
            .await?;
        
        if email_already_exists > 0 {
            return Ok(RegistrationOutcome::EMAIL_ALREADY_EXISTS);
        }
        
        // Generate verification code
        let (code, code_hash) = generate_verification_code()?;
        
        // Insert a new unverified user into the database
        let password_hash = generate_password_hash(password)?;
        let expires = time::add_days(time::now(), 1.0);
        let verification_id = sqlx::query_scalar!("INSERT INTO user_verification_codes (email, display_name, password_hash, code_hash, expires) VALUES (?, ?, ?, ?, ?) RETURNING id", email, display_name, password_hash, code_hash, expires)
            .fetch_one(&state.db)
            .await?;
        
        // Send confirmation email
        send_confirmation_email(state, email, verification_id, code.as_str())?;
        
        Ok(RegistrationOutcome {
            verification_id,
            error: None,
        })
    }
    
    pub async fn verify_account(state: &State, verification_id: i64, code: &str) -> Result<()> {
        // Get the user's details
        let user = sqlx::query_as!(UnverifiedUser, "SELECT id as verification_id, email, display_name, password_hash, code_hash FROM user_verification_codes WHERE id = ?", verification_id)
            .fetch_optional(&state.db)
            .await?
            .or_401_unauthorised()?;
        
        // Check that the submitted code is correct
        if !check_password(&user.code_hash, code)? {
            return Err(AppError::UNAUTHORIZED);
        }
        
        // Delete from the unverified table, and insert into the verified table
        sqlx::query!("DELETE FROM user_verification_codes WHERE id = ?", verification_id)
            .execute(&state.db)
            .await?;
        sqlx::query!("INSERT INTO USERS (email, display_name, password_hash) VALUES (?, ?, ?)", user.email, user.display_name, user.password_hash)
            .execute(&state.db)
            .await?;
        
        Ok(())
    }
    
    pub async fn delete_unverified_expired(state: &State) -> Result<()> {
        let now = time::now();
        
        log::info!("Deleting expired unverified users");
        sqlx::query!("DELETE FROM user_verification_codes WHERE expires <= ?", now)
            .execute(&state.db)
            .await?;
        
        Ok(())
    }
}