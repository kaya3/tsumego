use sqlx::types::JsonValue;

use crate::{result::Result, state::State};
use super::time;

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
        let tsumego = sqlx::query_as!(Self, "SELECT id, name, board, tree \"tree: JsonValue\" FROM tsumego WHERE id = ?", id)
            .fetch_optional(&state.db)
            .await?;
        
        log::info!("Tsumego tree: {:?}", tsumego.as_ref().map(|t| t.tree.clone()));
        Ok(tsumego)
    }
    
    /// Fetches a Tsumego from the database by its name, returning `None` if
    /// the name is not found.
    pub async fn get_by_name(state: &State, name: &str) -> Result<Option<Self>> {
        let tsumego = sqlx::query_as!(Self, "SELECT id, name, board, tree \"tree: JsonValue\" FROM tsumego WHERE name = ? LIMIT 1", name)
            .fetch_optional(&state.db)
            .await?;
        
        Ok(tsumego)
    }
    
    /// Fetches a vector of all Tsumego instances from the database.
    pub async fn get_all(state: &State) -> Result<Vec<Tsumego>> {
        let all_tsumego = sqlx::query_as!(Self, "SELECT id, name, board, tree \"tree: JsonValue\" FROM tsumego")
            .fetch_all(&state.db)
            .await?;
        
        Ok(all_tsumego)
    }
    
    pub async fn get_pending(state: &State, user_id: i64) -> Result<Vec<Tsumego>> {
        let now = time::now();
        let max_reviews = state.cfg.max_reviews_per_day;
        
        let pending = sqlx::query_as!(Self, "SELECT id, name, board, tree FROM tsumego WHERE id IN (SELECT tsumego_id FROM user_tsumego_stats WHERE user_id = ? AND review_due <= ?) LIMIT ?", user_id, now, max_reviews)
            .fetch_all(&state.db)
            .await?;
        
        Ok(pending)
    }
}
