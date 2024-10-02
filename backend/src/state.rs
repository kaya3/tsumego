use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

/// The `actix_web` application state, consisting of a database handle and the
/// application config.
pub type State = actix_web::web::Data<InnerState>;

pub struct InnerState {
    pub db: SqlitePool,
    pub cfg: Config,
}

type CowStr = std::borrow::Cow<'static, str>;

/// The application's config parameters, which are loaded from the environment
/// by the `from_env()` function.
#[derive(serde::Deserialize)]
pub struct Config {
    pub host_addr: CowStr,
    pub host_port: u16,
    
    pub database_url: CowStr,
    pub database_pool_size: u32,
    
    pub session_token_cookie_name: CowStr,
    pub session_duration_days: i64,
    pub session_renew_after_days: i64,
}

/// Loads the initial application state from the environment variables
/// specified in `.env`.
pub async fn from_env() -> State {
    let cfg: Config = envy::from_env()
        .unwrap_or_else(|err| {
            eprintln!("Failed to load config from environment: {err:?}");
            std::process::exit(1);
        });
    
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
