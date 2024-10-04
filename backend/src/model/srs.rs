#[derive(serde::Serialize, sqlx::Type)]
pub struct SrsState {
    /// The number of times the user has gotten this tsumego correct in a row.
    #[serde(rename = "streakLength")]
    pub streak_length: i64,
    
    /// The expected length of time, in days, between the user getting this
    /// tsumego right and them having a 90% chance to get it right. Also called
    /// the "stability".
    pub interval: f64,
    
    /// A number representing how easy this tsumego is for this user. A higher
    /// e-factor means the tsumego is easier.
    /// 
    /// The default e-factor for an unseen tsumego is 2.5; the e-factor is
    /// capped to at least 1.3, to avoid showing any tsumego too frequently.
    #[serde(rename = "eFactor")]
    pub e_factor: f64,
}

impl Default for SrsState {
    fn default() -> Self {
        Self {
            streak_length: 0,
            interval: 1.0,
            e_factor: 2.5,
        }
    }
}

