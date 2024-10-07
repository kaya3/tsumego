use std::collections::HashMap;
use string_template::Template;

use crate::{result::Result, state::State};

/// An error which might occur when trying to send an email.
#[derive(Debug)]
pub enum MailError {
    Address(lettre::address::AddressError),
    Compose(lettre::error::Error),
    Smtp(lettre::transport::smtp::Error),
}

impl From<lettre::address::AddressError> for MailError {
    fn from(err: lettre::address::AddressError) -> Self {
        MailError::Address(err)
    }
}

impl From<lettre::error::Error> for MailError {
    fn from(err: lettre::error::Error) -> Self {
        MailError::Compose(err)
    }
}

impl From<lettre::transport::smtp::Error> for MailError {
    fn from(err: lettre::transport::smtp::Error) -> Self {
        MailError::Smtp(err)
    }
}

/// Sends a confirmation email to the given email address, with a link to
/// activate the new user account.
pub fn send_confirmation_email(state: &State, email_address: &str, link: &str) -> Result<()> {
    let template_src = std::fs::read_to_string("templates/confirmation_email.txt")?;
    let template = Template::new(template_src.as_str());
    
    let mut args = HashMap::new();
    args.insert("link", link);
    
    let body = template.render(&args);
    send_email(state, email_address, "Verify your account", body)?;
    Ok(())
}

fn send_email(#[allow(unused)] state: &State, to: &str, subject: &str, body: String) -> Result<(), MailError> {
    // Only send mail in a release build; in a debug build, just log the
    // message which would be sent.
    #[cfg(debug_assertions)] {
        log::info!("Would send mail to {to}:\nSubject: {subject}\n{body}");
    }
    
    #[cfg(not(debug_assertions))] {
        use lettre::{
            message::header::ContentType,
            transport::smtp::SmtpTransport,
            Message,
            Transport,
        };
        
        let email = Message::builder()
            .from(state.cfg.email_from.parse()?)
            .to(to.parse()?)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body)?;
        
        SmtpTransport::unencrypted_localhost().send(&email)?;
    }
    
    Ok(())
}
