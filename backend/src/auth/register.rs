use crate::{
    auth::{hashing, send_confirmation_email},
    model::{time, User},
    result::{AppError, OrAppError, Result},
    routes::confirmation_link,
    state::State,
};

/// NIST recommends requiring a minimum password length of 8 characters.
const MINIMUM_PASSWORD_LENGTH: usize = 8;

struct UnverifiedUser {
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

/// Represents an error preventing registration of a new user, which should be
/// resolved by the user entering different details.
#[derive(serde::Serialize)]
pub enum RegistrationError {
    #[serde(rename = "Malformed email address")]
    MalformedEmail,
    #[serde(rename = "Email address already in use")]
    EmailAlreadyExists,
    #[serde(rename = "Failed to send an email with the verification link")]
    EmailFailed,
    #[serde(rename = "Please choose a display name")]
    MissingDisplayName,
    #[serde(rename = "Please choose a password of at least 8 characters")]
    PasswordTooShort,
}

impl User {
    /// Registers a new unverified user in the database, and sends an email to
    /// their address with a verification link. If registration is successful,
    /// the outcome contains the id of the new row in the `user_verification_codes`
    /// table. If registration is unsuccessful due to an  if registration
    /// is successful, or an error message otherwise.
    pub async fn register(state: &State, email: &str, display_name: &str, password: &str) -> Result<RegistrationOutcome> {
        fn error(error: RegistrationError) -> Result<RegistrationOutcome> {
            Ok(RegistrationOutcome {
                verification_id: -1,
                error: Some(error),
            })
        }
        
        // Repeat client-side checks, since we don't necessarily trust that
        // they were done.
        
        // Simple check to see if this looks like an email address. In general
        // the only sure way to validate an email address is to try sending
        // mail there.
        if !email.contains('@') {
            return error(RegistrationError::MalformedEmail);
        } else if display_name.is_empty() {
            return error(RegistrationError::MissingDisplayName);
        }
        
        // NIST recommend to require a minimum password length, but to NOT
        // require passwords to have certain compositions (e.g. include upper
        // and lowercase letters, special characters, etc.).
        // https://pages.nist.gov/800-63-3/sp800-63b.html#5111-memorized-secret-authenticators
        if password.len() < MINIMUM_PASSWORD_LENGTH {
            return error(RegistrationError::PasswordTooShort);
        }
        
        // Check if this email is already in use
        let email_already_exists = sqlx::query_scalar!(
            "SELECT COUNT(1) FROM (
                SELECT 1 FROM users WHERE email = ?
                UNION SELECT 1 FROM user_verification_codes WHERE email = ?
            )",
            email,
            email,
        )
            .fetch_one(&state.db)
            .await?;
        
        if email_already_exists > 0 {
            return error(RegistrationError::EmailAlreadyExists);
        }
        
        // Generate verification code
        let (code, code_hash) = hashing::generate_verification_code()?;
        
        // Insert a new unverified user into the database
        let password_hash = hashing::generate_password_hash(password)?;
        let expires = time::add_days(time::now(), 1.0);
        let verification_id = sqlx::query_scalar!(
            "INSERT INTO user_verification_codes
                (email, display_name, password_hash, code_hash, expires)
                VALUES (?, ?, ?, ?, ?)
                RETURNING id",
            email,
            display_name,
            password_hash,
            code_hash,
            expires,
        )
            .fetch_one(&state.db)
            .await?;
        
        // Send confirmation email
        let link = confirmation_link(state, verification_id, code.as_str());
        if let Err(e) = send_confirmation_email(state, email, link.as_str()) {
            log::info!("Failed to send verification link to <{email}>: {e}");
            
            // This code is unusable, since the verification link will not be
            // received; delete it from the database
            delete_verification_code_by_id(state, verification_id)
                .await?;
            
            return error(RegistrationError::EmailFailed);
        }
        
        Ok(RegistrationOutcome {
            verification_id,
            error: None,
        })
    }
    
    /// Verifies a new user by the verification code from the link in their
    /// confirmation email. If the code is correct, a verified user account
    /// is created in the database, and the verification code is deleted from
    /// the database.
    pub async fn verify_account(state: &State, verification_id: i64, code: &str) -> Result<()> {
        // Get the user's details
        let user = sqlx::query_as!(
            UnverifiedUser,
            "SELECT email, display_name, password_hash, code_hash
                FROM user_verification_codes
                WHERE id = ?",
            verification_id,
        )
            .fetch_optional(&state.db)
            .await?
            .or_401_unauthorised()?;
        
        // Check that the submitted code is correct
        if !hashing::check_password(&user.code_hash, code)? {
            return Err(AppError::UNAUTHORIZED);
        }
        
        // Delete from the unverified table, and insert into the table of
        // verified users
        delete_verification_code_by_id(state, verification_id)
            .await?;
        sqlx::query!(
            "INSERT INTO USERS
                (email, display_name, password_hash)
                VALUES (?, ?, ?)",
            user.email,
            user.display_name,
            user.password_hash,
        )
            .execute(&state.db)
            .await?;
        
        Ok(())
    }
    
    /// Deletes expired rows from the `user_verification_codes` table. This
    /// function will be called periodically.
    pub async fn delete_unverified_expired(state: &State) -> Result<()> {
        let now = time::now();
        
        log::info!("Deleting expired unverified users");
        sqlx::query!(
            "DELETE FROM user_verification_codes WHERE expires <= ?",
            now,
        )
            .execute(&state.db)
            .await?;
        
        Ok(())
    }
}

async fn delete_verification_code_by_id(state: &State, id: i64) -> Result<()> {
    sqlx::query!(
        "DELETE FROM user_verification_codes WHERE id = ?",
        id,
    )
        .execute(&state.db)
        .await?;
    
    Ok(())
}