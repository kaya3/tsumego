use crate::{
    model::{time, User},
    result::AppError,
    state::State,
};

impl authlogic::AppTypes for State {
    type DateTime = time::DateTime;
    type ID = i64;
    type User = User;
    type CustomChallenge = authlogic::mail::NoCustomChallenges;
    type Error = AppError;
}

impl authlogic::AppConfig for State {
    fn session_expire_after_hours(&self) -> u64 {
        24 * self.cfg.session_duration_days as u64
    }

    fn session_renew_after_hours(&self) -> u64 {
        24 * self.cfg.session_renew_after_days as u64
    }

    fn session_token_cookie_name(&self) -> &str {
        &self.cfg.session_token_cookie_name
    }
}

impl authlogic::App for State {
    fn time_now(&self) -> time::DateTime {
        time::now()
    }
}
