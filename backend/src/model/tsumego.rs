use sqlx::types::JsonValue;

use crate::state::State;

#[derive(serde::Serialize, sqlx::FromRow)]
pub struct Tsumego {
    pub id: i64,
    pub name: String,
    pub board: String,
    pub tree: JsonValue,
}

impl Tsumego {
    pub async fn require_by_id(state: &State, id: i64) -> Result<Self, sqlx::Error> {
        Self::get_by_id(state, id).await?
            .map_or(Err(sqlx::Error::RowNotFound), Ok)
    }
    
    pub async fn get_by_id(state: &State, id: i64) -> Result<Option<Self>, sqlx::Error> {
        let tsumego = sqlx::query_as!(Self, "SELECT id, name, board, tree \"tree: JsonValue\" FROM tsumego WHERE id = ?", id)
            .fetch_optional(&state.db)
            .await?;
        
        log::info!("Tsumego tree: {:?}", tsumego.as_ref().map(|t| t.tree.clone()));
        Ok(tsumego)
    }
    
    pub async fn get_by_name(state: &State, name: &str) -> Result<Option<Self>, sqlx::Error> {
        Ok(sqlx::query_as!(Self, "SELECT id, name, board, tree \"tree: JsonValue\" FROM tsumego WHERE name = ? LIMIT 1", name)
            .fetch_optional(&state.db)
            .await?)
    }
    
    pub async fn get_all(state: &State) -> Result<Vec<Tsumego>, sqlx::Error> {
        Ok(sqlx::query_as!(Self, "SELECT id, name, board, tree \"tree: JsonValue\" FROM tsumego")
            .fetch_all(&state.db)
            .await?)
    }
}
