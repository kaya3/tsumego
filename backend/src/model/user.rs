use crate::{
    model::time,
    result::Result,
    state::State,
};

#[derive(Clone, serde::Serialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub email: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "isAdmin")]
    pub is_admin: bool,
}

impl User {
    /// Retrieves the user with the given id, or returns an error if there is
    /// no user with that id.
    pub async fn require_by_id(state: &State, id: i64) -> Result<Self> {
        Self::get_by_id(state, id)
            .await?
            .map_or(Err(sqlx::Error::RowNotFound.into()), Ok)
    }
    
    /// Retrieves the user with the given id, if they exist.
    pub async fn get_by_id(state: &State, id: i64) -> Result<Option<Self>> {
        let user = sqlx::query_as!(
            Self,
            "SELECT id, email, display_name, is_admin FROM users
                WHERE id = ?",
            id,
        )
            .fetch_optional(&state.db)
            .await?;
        
        Ok(user)
    }
    
    /// Retrieves the user with the given email address, if they exist.
    pub async fn get_by_email(state: &State, email: &str) -> Result<Option<Self>> {
        // The `users.email` column is declared with `NOCASE`, so there is no
        // need to normalise before querying
        let user = sqlx::query_as!(
            Self,
            "SELECT id, email, display_name, is_admin FROM users
                WHERE email = ?
                LIMIT 1",
            email,
        )
            .fetch_optional(&state.db)
            .await?;
        
        Ok(user)
    }
    
    /// Retrieves a vector of all users in the database.
    pub async fn get_all(state: &State) -> Result<Vec<Self>> {
        let users = sqlx::query_as!(
            Self,
            "SELECT id, email, display_name, is_admin FROM users
                ORDER by id",
        )
            .fetch_all(&state.db)
            .await?;
        
        Ok(users)
    }
}

/// Details about a user including their statistics for today's study.
#[derive(serde::Serialize)]
pub struct UserDetails {
    #[serde(flatten)]
    pub user: User,
    #[serde(rename = "reviewsDueToday")]
    pub reviews_due_today: i64,
    #[serde(rename = "reviewsDoneToday")]
    pub reviews_done_today: i64,
}

impl UserDetails {
    /// Gets statistics about the user's study for today.
    pub async fn get_for_user(state: &State, user: User) -> Result<Self> {
        let now = time::now();
        let start_of_day = time::start_of_day(now);
        
        struct Details {
            due_today: Option<i32>,
            done_today: Option<i32>,
        }
        
        let details = sqlx::query_as!(
            Details,
            "SELECT
                (SELECT COUNT(1) FROM user_tsumego_stats
                    WHERE user_id = ? AND review_due <= ?
                ) as due_today,
                (SELECT COUNT(1) from user_tsumego_reviews
                    WHERE user_id = ? AND review_date >= ?
                ) as done_today",
            user.id,
            now,
            user.id,
            start_of_day,
        )
            .fetch_one(&state.db)
            .await?;
        
        Ok(Self {
            user,
            reviews_due_today: details.due_today.unwrap_or(0) as i64,
            reviews_done_today: details.done_today.unwrap_or(0) as i64,
        })
    }
}