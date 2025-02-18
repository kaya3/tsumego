use std::collections::HashMap;
use string_template::Template;

use authlogic::{
    mail::{Challenge, Notification},
    AppMailer,
    Secret,
};

use crate::{
    model::User,
    result::Result,
    state::State,
};

impl AppMailer for State {
    async fn send_notification(&self, user: &User, notification: Notification) -> Result<()> {
        let mut args = HashMap::new();
        let (template_path, subject) = match notification {
            Notification::UserRegistered {ref temporary_password} => {
                args.insert("temporary_password", temporary_password.expose());
                ("templates/user_registered.txt", "Your account has been created")
            },
            Notification::PasswordChanged => {
                ("templates/password_changed.txt", "Your password has been changed")
            },
        };
        
        compose_and_send(self, user, subject, template_path, args)
    }
    
    async fn send_challenge(&self, user: &User, challenge: Challenge<State>, code: Secret) -> Result<()> {
        let link = crate::routes::confirmation_link(self, code);
        let mut args = HashMap::new();
        args.insert("link", link.as_str());
        
        let (template_path, subject) = match challenge {
            Challenge::LogIn => {
                ("templates/log_in.txt", "Log in")
            },
            Challenge::ResetPassword => {
                ("templates/reset_password.txt", "Reset your password")
            },
            Challenge::VerifyNewUser => {
                ("templates/verify_account.txt", "Verify your account")
            },
            Challenge::Custom(c) => c.never_happens(),
        };
        
        compose_and_send(self, user, subject, template_path, args)
    }
}

/// An error which might occur when trying to send an email.
#[derive(Debug)]
pub enum MailError {
    Address(lettre::address::AddressError),
    Compose(lettre::error::Error),
    Smtp(lettre::transport::smtp::Error),
}

fn compose_and_send<'a>(state: &State, user: &'a User, subject: &str, template_path: &str, mut args: HashMap<&'static str, &'a str>) -> Result<()> {
    args.insert("display_name", user.display_name.as_str());
    let body = compose_email(template_path, args)?;
    send_email(state, &user.email, subject, body)?;
    Ok(())
}

fn compose_email(template_path: &str, args: HashMap<&str, &str>) -> Result<String> {
    let template_src = std::fs::read_to_string(template_path)?;
    let template = Template::new(template_src.as_str());
    Ok(template.render(&args))
}

fn send_email(state: &State, to: &str, subject: &str, body: String) -> Result<(), MailError> {
    // Only send mail in a release build; in a debug build, just log the
    // message which would be sent.
    if cfg!(debug_assertions) {
        log::info!("Would send mail to {to}:\nSubject: {subject}\n{body}");
    } else {
        use lettre::{
            message::header::ContentType,
            transport::smtp::SmtpTransport,
            Message,
            Transport,
        };
        
        let address_from = state.cfg.email_from
            .parse()
            .map_err(MailError::Address)?;
        
        let address_to = to
            .parse()
            .map_err(MailError::Address)?;

        let email = Message::builder()
            .from(address_from)
            .to(address_to)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body)
            .map_err(MailError::Compose)?;
        
        SmtpTransport::unencrypted_localhost()
            .send(&email)
            .map_err(MailError::Smtp)?;
    }
    
    Ok(())
}
