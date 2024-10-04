/// The state of a user's memory of a tsumego, used in a spaced repetition
/// system (SRS). The SRS attempts to predict the optimal time spacing for
/// reviews of an item.
/// 
/// A user's SRS state for a tsumego is updated on each review, based on the
/// time elapsed since the previous review, and the review's grade. The grade
/// indicates how easy or difficult the tsumego was for the user on this
/// review.
/// 
/// https://freshcardsapp.com/srs/write-your-own-algorithm.html
#[derive(Clone, serde::Serialize, sqlx::Type)]
pub struct SrsState {
    /// The number of times the user has reviewed this tsumego.
    #[serde(rename = "numReviews")]
    pub num_reviews: i64,
    
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
            num_reviews: 0,
            streak_length: 0,
            interval: 1.0,
            e_factor: 2.5,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, serde::Deserialize)]
pub enum Grade {
    Again = 0,
    Hard = 1,
    Good = 2,
    Easy = 3,
}

impl SrsState {
    /// Returns a new SRS state for the user's first review of a tsumego, when
    /// there is no priod state.
    pub fn after_first_review(grade: Grade) -> Self {
        Self::default()
            .update_on_review(0.0, grade)
    }
    
    /// Updates the SRS state based on the timing and grade of a review.
    pub fn update_on_review(&self, days_since_last_review: f64, grade: Grade) -> Self {
        // The algorithm here is adapted from Allen Ussher's "Anki-like"
        // algorithm, given here:
        // https://freshcardsapp.com/srs/simulator/
        // 
        // Some notes:
        // - An item is in the "learning" or "relearning" phase when
        //   `streak_length <= 2`. For such items, the review interval is less
        //   than one day, and the e-factor is not updated.
        // - No random "fuzz" is added to the interval in this function; the
        //   fuzz will be added to the date for the next review later.
        
        /// A time of one minute, in days. 
        const ONE_MINUTE: f64 = 1.0 / (24.0 * 60.0);
        
        const FIRST_INTERVAL: f64 = ONE_MINUTE;
        const SECOND_INTERVAL: f64 = 10.0 * ONE_MINUTE;
        const THIRD_INTERVAL: f64 = 1.0;
        const EASY_INTERVAL: f64 = 4.0;
        
        fn clamp_e_factor(e_factor: f64) -> f64 {
            f64::max(1.3, e_factor)
        }
        
        let previous = self;
        let mut next = self.clone();
        
        let is_learning = previous.streak_length <= 2;
        
        next.num_reviews += 1;
        
        if grade == Grade::Again {
            // Failed
            next.streak_length = 0;
            next.interval = FIRST_INTERVAL;
            
            if !is_learning {
                next.e_factor = clamp_e_factor(previous.e_factor - 0.2);
            }
        } else {
            // Passed
            next.streak_length += 1;
            
            if is_learning {
                // Learning phase: use fixed intervals
                next.interval = if grade == Grade::Easy {
                    EASY_INTERVAL
                } else if previous.streak_length == 0 {
                    FIRST_INTERVAL
                } else if previous.streak_length == 1 {
                    SECOND_INTERVAL
                } else {
                    THIRD_INTERVAL
                };
            } else {
                // Reviewing phase: use dynamic intervals
                let lateness_days = f64::max(0.0, days_since_last_review - previous.interval);
                let lateness_bonus = lateness_days * match grade {
                    Grade::Easy => 1.0,
                    Grade::Good => 0.5,
                    _ => 0.25,
                };
                
                let inv_score = 3.0 - (grade as usize as f64);
                let e_factor = previous.e_factor + (0.1 - inv_score * (0.08 + inv_score * 0.02));
                let working_e_factor = e_factor + match grade {
                    Grade::Easy => 0.15,
                    Grade::Good => 0.0,
                    _ => -0.15,
                };
                
                next.interval = (previous.interval + lateness_bonus) * clamp_e_factor(working_e_factor);
                next.e_factor = clamp_e_factor(e_factor);
            }
        }
        
        next
    }
}

/// Tests for the spaced repetition system algorithm. We don't want to test
/// that the `interval` and `e_factor` fields equal exact numbers, since the
/// algorithm is subject to change; instead, we test that the results are
/// sensible and self-consistent.
#[cfg(test)]
mod test {
    use super::{Grade, SrsState};
    
    #[test]
    fn on_first_review() {
        let initial = SrsState::default();
        
        let result_easy = initial.update_on_review(0.0, Grade::Easy);
        let result_good = initial.update_on_review(0.0, Grade::Good);
        let result_hard = initial.update_on_review(0.0, Grade::Hard);
        let result_again = initial.update_on_review(0.0, Grade::Again);
        
        assert_eq!(1, result_easy.num_reviews);
        assert_eq!(1, result_easy.streak_length);
        
        assert_eq!(1, result_good.num_reviews);
        assert_eq!(1, result_good.streak_length);
        
        assert_eq!(1, result_hard.num_reviews);
        assert_eq!(1, result_hard.streak_length);
        
        assert_eq!(1, result_again.num_reviews);
        assert_eq!(0, result_again.streak_length);
        
        // "Easy" shouldn't be worse than "Good"
        assert!(result_easy.interval >= result_good.interval);
        
        // "Good" shouldn't be worse than "Hard"
        assert!(result_good.interval >= result_hard.interval);
        
        // "Hard" shouldn't be worse than "Again"
        assert!(result_hard.interval >= result_again.interval);
    }
    
    #[test]
    fn after_several_reviews() {
        let initial = SrsState::default()
            .update_on_review(1.0, Grade::Good)
            .update_on_review(4.0, Grade::Again)
            .update_on_review(0.1, Grade::Good)
            .update_on_review(1.0, Grade::Good)
            .update_on_review(2.0, Grade::Easy);
        
        assert_eq!(5, initial.num_reviews);
        assert_eq!(3, initial.streak_length);
        
        let result_easy = initial.update_on_review(initial.interval, Grade::Easy);
        let result_good = initial.update_on_review(initial.interval, Grade::Good);
        let result_hard = initial.update_on_review(initial.interval, Grade::Hard);
        let result_again = initial.update_on_review(initial.interval, Grade::Again);
        
        assert_eq!(6, result_easy.num_reviews);
        assert_eq!(4, result_easy.streak_length);
        
        assert_eq!(6, result_good.num_reviews);
        assert_eq!(4, result_good.streak_length);
        
        assert_eq!(6, result_hard.num_reviews);
        assert_eq!(4, result_hard.streak_length);
        
        assert_eq!(6, result_again.num_reviews);
        assert_eq!(0, result_again.streak_length);
        
        // "Easy" shouldn't be worse than "Good"
        assert!(result_easy.interval >= result_good.interval);
        assert!(result_easy.e_factor >= result_good.e_factor);
        
        // "Good" shouldn't be worse than "Hard"
        assert!(result_good.interval >= result_hard.interval);
        assert!(result_good.e_factor >= result_hard.e_factor);
        
        // "Hard" shouldn't be worse than "Again"
        assert!(result_hard.interval >= result_again.interval);
        assert!(result_hard.e_factor >= result_again.e_factor);
        
        // "Easy" shouldn't be worse than initial state
        assert!(result_easy.interval >= initial.interval);
        assert!(result_easy.e_factor >= initial.e_factor);
        
        // "Good" shouldn't be worse than initial state
        assert!(result_good.interval >= initial.interval);
        assert!(result_good.e_factor >= initial.e_factor);
        
        // "Hard" shouldn't worsen interval, but shouldn't improve e-factor
        // compared to initial state
        assert!(result_hard.interval >= initial.interval);
        assert!(result_hard.e_factor <= initial.e_factor);
        
        // "Again" shouldn't be better than initial state
        assert!(result_again.interval <= initial.interval);
        assert!(result_again.e_factor <= initial.e_factor);
    }
    
    #[test]
    fn late_review() {
        let initial = SrsState::default()
            .update_on_review(1.0, Grade::Good)
            .update_on_review(2.0, Grade::Good)
            .update_on_review(4.0, Grade::Good);
        
        let on_time = initial.update_on_review(initial.interval, Grade::Easy);
        let late = initial.update_on_review(initial.interval + 5.0, Grade::Easy);
        
        // A late success is at least as good as an on-time success, because
        // the user went longer than expected without forgetting
        assert!(late.interval >= on_time.interval);
        assert!(late.e_factor >= on_time.e_factor);
    }
}
