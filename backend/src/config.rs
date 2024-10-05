type CowStr = std::borrow::Cow<'static, str>;

/// The application's config parameters, which are loaded from the environment
/// by the `from_env()` function.
#[derive(serde::Deserialize)]
pub struct Config {
    pub host_addr: CowStr,
    pub host_port: u16,
    
    pub base_url: CowStr,
    
    pub database_url: CowStr,
    pub database_pool_size: u32,
    
    pub session_token_cookie_name: CowStr,
    pub session_duration_days: i64,
    pub session_renew_after_days: i64,
    
    pub max_reviews_per_day: i64,
    pub srs_interval_fuzz_factor: f64,
}

impl Config {
    pub fn get_from_env() -> Self {
        envy::from_env()
            .unwrap_or_else(|err| {
                eprintln!("Failed to load config from environment: {err:?}");
                std::process::exit(1);
            })
    }
}