use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppConfig {
    pub database_url: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let cfg = config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()
            .unwrap();

        cfg.try_deserialize().unwrap()
    }
}

pub static ENVS: Lazy<AppConfig> = Lazy::new(|| AppConfig::from_env());
