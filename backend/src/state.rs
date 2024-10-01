pub type State = actix_web::web::Data<InnerState>;

pub struct InnerState {
    pub cfg: Config,
}

type CowStr = std::borrow::Cow<'static, str>;

#[derive(serde::Deserialize)]
pub struct Config {
    pub host_addr: CowStr,
    pub host_port: u16,
}

pub async fn from_env() -> State {
    let cfg: Config = envy::from_env()
        .unwrap_or_else(|err| {
            eprintln!("Failed to load config from environment: {err:?}");
            std::process::exit(1);
        });
    
    State::new(InnerState {
        cfg,
    })
}
