use rand::Rng;

use crate::{state::State, result::Result};
use super::{Grade, SrsState};

#[derive(serde::Serialize)]
pub struct UserTsumegoStats {
    id: i64,
    
    /// The id of the User these stats belong to.
    #[serde(rename = "userID")]
    user_id: i64,
    
    /// The id of the Tsumego these stats are for.
    #[serde(rename = "tsumegoID")]
    tsumego_id: i64,
    
    /// The last time this user reviewed this tsumego, in UTC.
    #[serde(rename = "lastReviewDate")]
    last_review_date: chrono::NaiveDateTime,
    
    /// The time at or after which this user should be prompted on this tsumego
    /// again, in UTC. `None` indicates the tsumego has been reviewed in the
    /// past but is not currently in rotation, and shouldn't be prompted.
    #[serde(rename = "reviewDue")]
    review_due: Option<chrono::NaiveDateTime>,
    
    /// The spaced repetition system (SRS) state representing this user's
    /// memory of this tsumego.
    #[serde(rename = "srsState")]
    srs_state: SrsState,
}

/// This type is used for database queries of the `user_tsumego_stats` table,
/// but should then be immediately converted to the type `UserTsumegoStats`,
/// e.g. by calling `from(...)` or `into()`.
/// 
/// Unfortunately, SQLx can't construct a `UserTsumegoStats` from a query
/// directly, since the columns `streak_length`, `interval` and `e_factor` must
/// be combined into an `SrsState` struct.
struct FlatStats {
    id: i64,
    user_id: i64,
    tsumego_id: i64,
    last_review_date: chrono::NaiveDateTime,
    review_due: Option<chrono::NaiveDateTime>,
    num_reviews: i64,
    streak_length: i64,
    interval: f64,
    e_factor: f64,
}

impl From<FlatStats> for UserTsumegoStats {
    fn from(flat: FlatStats) -> Self {
        Self {
            id: flat.id,
            user_id: flat.user_id,
            tsumego_id: flat.tsumego_id,
            last_review_date: flat.last_review_date,
            review_due: flat.review_due,
            srs_state: SrsState {
                num_reviews: flat.num_reviews,
                streak_length: flat.streak_length,
                interval: flat.interval,
                e_factor: flat.e_factor,
            },
        }
    }
}

impl UserTsumegoStats {
    pub async fn get_by_id(state: &State, id: i64) -> Result<Option<Self>> {
        let stats = sqlx::query_as!(FlatStats, "SELECT * FROM user_tsumego_stats WHERE id = ? LIMIT 1", id)
            .fetch_optional(&state.db)
            .await?
            .map(Self::from);
        
        Ok(stats)
    }
    
    pub async fn get(state: &State, user_id: i64, tsumego_id: i64) -> Result<Option<Self>> {
        let stats = sqlx::query_as!(FlatStats, "SELECT * FROM user_tsumego_stats WHERE user_id = ? AND tsumego_id = ? LIMIT 1", user_id, tsumego_id)
            .fetch_optional(&state.db)
            .await?
            .map(Self::from);
        
        Ok(stats)
    }
    
    pub async fn update_on_review(state: &State, user_id: i64, tsumego_id: i64, grade: Grade) -> Result<Self> {
        let now = chrono::Utc::now().naive_utc();
        
        let stats = Self::get(state, user_id, tsumego_id)
            .await?;
        
        let srs_state = match stats.as_ref() {
            Some(prior) => {
                // The user already has stats for this tsumego
                let days_since_last_review = delta_days(prior.last_review_date, now);
                prior.srs_state
                    .update_on_review(days_since_last_review, grade)
            },
            None => {
                // This is the user's first review of this tsumego
                SrsState::after_first_review(grade)
            },
        };
        
        let review_due = if matches!(stats, Some(Self {review_due: None, ..})) {
            // Don't set a new due date if this tsumego is out of rotation
            None
        } else {
            // Add random fuzz to the interval. This prevents "bunching up";
            // otherwise, tsumego prompted on the same day would continue to be
            // prompted together in the future.
            let fuzz_factor = state.cfg.srs_interval_fuzz_factor;
            let fuzz_range = (1.0 - fuzz_factor)..(1.0 + fuzz_factor);
            let fuzz: f64 = rand::thread_rng().gen_range(fuzz_range);
            
            Some(add_days(now, srs_state.interval * fuzz))
        };
        
        let id = sqlx::query_scalar!("INSERT OR REPLACE INTO user_tsumego_stats (user_id, tsumego_id, last_review_date, review_due, streak_length, interval, e_factor) VALUES (?, ?, ?, ?, ?, ?, ?) RETURNING id", user_id, tsumego_id, now, review_due, srs_state.streak_length, srs_state.interval, srs_state.e_factor)
            .fetch_one(&state.db)
            .await?;
        
        let new_stats = match stats {
            None => Self {
                id,
                user_id,
                tsumego_id,
                last_review_date: now,
                review_due,
                srs_state,
            },
            Some(stats) => Self {
                last_review_date: now,
                review_due,
                srs_state,
                ..stats
            },
        };
        
        Ok(new_stats)
    }
}

const SECONDS_PER_DAY: f64 = 60.0 * 60.0 * 24.0;

fn add_days(from: chrono::NaiveDateTime, delta_days: f64) -> chrono::NaiveDateTime {
    let delta_seconds = SECONDS_PER_DAY * delta_days;
    from + chrono::TimeDelta::seconds(delta_seconds as i64)
}

fn delta_days(from: chrono::NaiveDateTime, to: chrono::NaiveDateTime) -> f64 {
    let delta_seconds = (to - from).num_seconds();
    (delta_seconds as f64) * SECONDS_PER_DAY
}