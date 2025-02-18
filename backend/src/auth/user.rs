use crate::{
    model::User,
    result::{AppError, Result},
    state::State,
};

impl authlogic::UserID<i64> for User {
    fn id(&self) -> i64 {
        self.id
    }

    fn set_id(&mut self, new_id: i64) {
        self.id = new_id;
    }
}

#[derive(serde::Serialize)]
pub struct RegistrationOutcome {
    #[serde(rename = "newUserID")]
    pub new_user_id: i64,
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

/// Registers a new unverified user in the database, and sends an email to
/// their address with a verification link. If registration is successful,
/// the outcome contains the id of the new row in the `user_verification_codes`
/// table. If registration is unsuccessful due to an  if registration
/// is successful, or an error message otherwise.
pub async fn register(state: &State, user: User, password: authlogic::Secret) -> Result<RegistrationOutcome> {
    fn error_outcome(error: RegistrationError) -> Result<RegistrationOutcome> {
        Ok(RegistrationOutcome {
            new_user_id: -1,
            error: Some(error),
        })
    }
    
    // Repeat client-side checks, since we don't necessarily trust that
    // they were done.
    
    // Simple check to see if this looks like an email address. In general
    // the only sure way to validate an email address is to try sending
    // mail there.
    if !user.email.contains('@') {
        return error_outcome(RegistrationError::MalformedEmail);
    } else if user.display_name.is_empty() {
        return error_outcome(RegistrationError::MissingDisplayName);
    }
    
    // Check if this email is already in use
    let email_already_exists = sqlx::query_scalar!(
        "SELECT COUNT(1) FROM users WHERE email = ?",
        user.email,
    )
        .fetch_one(&state.db)
        .await?;
    
    if email_already_exists > 0 {
        return error_outcome(RegistrationError::EmailAlreadyExists);
    }
    
    let result = authlogic::register_new_user(state, user, password).await;
    
    match result {
        Ok(user) => Ok(RegistrationOutcome {
            new_user_id: user.id,
            error: None,
        }),
        
        Err(AppError::Auth(authlogic::Error::PasswordTooShort)) => error_outcome(RegistrationError::PasswordTooShort),
        Err(AppError::Mail(_)) => error_outcome(RegistrationError::EmailFailed),
        Err(e) => Err(e),
    }
}
