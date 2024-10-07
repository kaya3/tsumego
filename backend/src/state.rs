use sqlx::sqlite::{
    SqlitePool,
    SqlitePoolOptions,
};

use crate::config::Config;

/// The `actix_web` application state, consisting of a database handle and the
/// application config.
pub type State = actix_web::web::Data<InnerState>;

pub struct InnerState {
    pub db: SqlitePool,
    pub cfg: Config,
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
    
    State::new(InnerState {
        db,
        cfg,
    })
}
