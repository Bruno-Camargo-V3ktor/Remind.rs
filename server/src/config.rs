use figment::{
    Figment,
    providers::{Env, Format, Json},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct ServerConfig {
    pub addr: String,
    pub hostname: String,
    pub workers: usize,
    pub keep_alive_secs: u64,
    pub storage_dir: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SurrealDbConfig {
    pub url: String,
    pub username: String,
    pub password: String,
    pub namespace: String,
    pub database: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ConfigApp {
    pub server: ServerConfig,
    pub surreal_db: Option<SurrealDbConfig>,
}

pub fn load_config() -> ConfigApp {
    Figment::new()
        .merge(Env::prefixed("REMIND_").split("__"))
        .merge(Json::file("config.json"))
        .extract::<ConfigApp>()
        .expect("Invalid Configuration")
}
