use sqlx::types::JsonValue;

use crate::{
    model::time,
    result::Result,
    state::State,
};

/// Data for a tsumego. The board and variation tree are sent to the client,
/// and not otherwise used in the backend.
#[derive(serde::Serialize, sqlx::FromRow)]
pub struct Tsumego {
    pub id: i64,
    pub name: String,
    pub board: String,
    // This field has to be of type `JsonValue` instead of `String`, so that it
    // doesn't get an extra pair of quotes when a `Tsumego` is serialised.
    pub tree: JsonValue,
}

impl Tsumego {
    /// Fetches a Tsumego from the database by its id, returning an error if
    /// the id is not found.
    pub async fn require_by_id(state: &State, id: i64) -> Result<Self> {
        Self::get_by_id(state, id)
            .await?
            .map_or(Err(sqlx::Error::RowNotFound.into()), Ok)
    }
    
    /// Fetches a Tsumego from the database by its id, returning `None` if the
    /// id is not found.
    pub async fn get_by_id(state: &State, id: i64) -> Result<Option<Self>> {
        let tsumego = sqlx::query_as!(
            Self,
            "SELECT id, name, board, tree \"tree: JsonValue\" FROM tsumego
                WHERE id = ?",
            id,
        )
            .fetch_optional(&state.db)
            .await?;
        
        log::info!("Tsumego tree: {:?}", tsumego.as_ref().map(|t| t.tree.clone()));
        Ok(tsumego)
    }
    
    /// Fetches up to `limit` randomly-selected tsumego from the database.
    pub async fn get_random(state: &State, limit: i64) -> Result<Vec<Tsumego>> {
        let tsumego = sqlx::query_as!(
            Self,
            "SELECT id, name, board, tree \"tree: JsonValue\" FROM tsumego
                ORDER BY RANDOM()
                LIMIT ?",
            limit,
        )
            .fetch_all(&state.db)
            .await?;
        
        Ok(tsumego)
    }
    
    /// Fetches up to `limit` randomly-selected tsumego from the database,
    /// which haven't yet been studied by this user.
    pub async fn get_random_unstudied(state: &State, user_id: i64, limit: i64) -> Result<Vec<Tsumego>> {
        let unstudied_tsumego = sqlx::query_as!(
            Self,
            "SELECT id, name, board, tree \"tree: JsonValue\" FROM tsumego
                WHERE id NOT IN (
                    SELECT tsumego_id FROM user_tsumego_stats
                        WHERE user_id = ?
                )
                ORDER BY RANDOM()
                LIMIT ?",
            user_id,
            limit,
        )
            .fetch_all(&state.db)
            .await?;
        
        Ok(unstudied_tsumego)
    }
    
    /// Fetches tsumego from the database which are due for review by this
    /// user. The number of tsumego is capped by the environment variable
    /// `MAX_PROBLEMS_AT_ONCE`.
    pub async fn get_pending(state: &State, user_id: i64) -> Result<Vec<Tsumego>> {
        let now = time::now();
        let max_reviews = state.cfg.max_problems_at_once;
        
        let pending = sqlx::query_as!(
            Self,
            "SELECT id, name, board, tree \"tree: JsonValue\" FROM tsumego
                WHERE id IN (
                    SELECT tsumego_id FROM user_tsumego_stats
                        WHERE user_id = ? AND review_due <= ?
                )
                LIMIT ?",
            user_id,
            now,
            max_reviews,
        )
            .fetch_all(&state.db)
            .await?;
        
        Ok(pending)
    }
}
