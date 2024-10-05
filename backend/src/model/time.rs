use chrono::{NaiveDateTime, TimeDelta, Utc};

/// The type of datetime used in this application. `chrono::NaiveDateTime`
/// is "timezone-less", but `chrono::Utc` doesn't work with `sqlx`. So the
/// application uses `NaiveDateTime` but ensures that all absolute times are
/// UTC.
pub type DateTime = NaiveDateTime;

pub fn now() -> DateTime {
    Utc::now().naive_utc()
}

const SECONDS_PER_DAY: f64 = 60.0 * 60.0 * 24.0;

pub fn add_days(from: DateTime, delta_days: f64) -> DateTime {
    let delta_seconds = SECONDS_PER_DAY * delta_days;
    from + TimeDelta::seconds(delta_seconds as i64)
}

pub fn delta_days(from: DateTime, to: DateTime) -> f64 {
    let delta_seconds = (to - from).num_seconds();
    (delta_seconds as f64) / SECONDS_PER_DAY
}

/// Returns the time at the start of the given day.
pub fn start_of_day(time: DateTime) -> DateTime {
    // Possible feature for later: make days tick over at a time based on the
    // user's preferences
    time.date()
        .and_hms_opt(0, 0, 0)
        .expect("00:00:00 is a valid time on any day")
}
