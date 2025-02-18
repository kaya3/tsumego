use std::{
    convert::Infallible, ops::Deref, sync::Arc
};

use actix_web::FromRequest;
use sqlx::sqlite::{
    SqlitePool,
    SqlitePoolOptions,
};

use crate::config::Config;

/// The `actix_web` application state, consisting of a database handle and the
/// application config.
#[derive(Clone)]
pub struct State(Arc<InnerState>);

pub struct InnerState {
    pub db: SqlitePool,
    pub cfg: Config,
}

impl Deref for State {
    type Target = InnerState;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromRequest for State {
    type Error = Infallible;
    type Future = std::future::Ready<Result<Self, Infallible>>;

    fn from_request(req: &actix_web::HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let data = req.app_data::<State>()
            .expect("State should be available from app data")
            .clone();

        std::future::ready(Ok(data))
    }
}

/// Loads the initial application state from the environment variables
/// specified in `.env`.
pub async fn from_env() -> State {
    let cfg = Config::get_from_env();
    
    let db = SqlitePoolOptions::new()
        .max_connections(cfg.database_pool_size)
        .connect(&cfg.database_url)
        .await
        .unwrap_or_else(|err| {
            eprintln!("Failed to connect to the database: {err:?}");
            std::process::exit(1);
        });
    
    State(Arc::new(InnerState {
        db,
        cfg,
    }))
}
