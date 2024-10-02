use sqlx::types::JsonValue;

use crate::result::Result;
use crate::state::State;

/// Data for a tsumego. The board and variation tree are sent to the client,
/// and not otherwise used in the backend.
#[derive(serde::Serialize, sqlx::FromRow)]
pub struct Tsumego {
    pub id: i64,
    pub name: String,
    pub board: String,
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
        Ok(sqlx::query_as!(Self, "SELECT id, name, board, tree \"tree: JsonValue\" FROM tsumego WHERE name = ? LIMIT 1", name)
            .fetch_optional(&state.db)
            .await?)
    }
    
    /// Fetches a vector of all Tsumego instances from the database.
    pub async fn get_all(state: &State) -> Result<Vec<Tsumego>> {
        Ok(sqlx::query_as!(Self, "SELECT id, name, board, tree \"tree: JsonValue\" FROM tsumego")
            .fetch_all(&state.db)
            .await?)
    }
}
