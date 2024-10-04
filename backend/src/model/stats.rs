use crate::{state::State, result::Result};
use super::SrsState;

#[derive(serde::Serialize)]
pub struct UserTsumegoStats {
    id: i64,
    #[serde(rename = "userID")]
    user_id: i64,
    #[serde(rename = "tsumegoID")]
    tsumego_id: i64,
    #[serde(rename = "inRotation")]
    in_rotation: bool,
    #[serde(rename = "reviewDue")]
    review_due: chrono::NaiveDateTime,
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
    in_rotation: bool,
    review_due: chrono::NaiveDateTime,
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
            in_rotation: flat.in_rotation,
            review_due: flat.review_due,
            srs_state: SrsState {
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
}
