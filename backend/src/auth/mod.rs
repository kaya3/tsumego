mod db;
mod mail;
mod state;
mod user;

pub use mail::MailError;
pub use user::register;

pub type MaybeAuth = authlogic::MaybeAuth<crate::state::State>;
pub type Auth = authlogic::Auth<crate::state::State>;
