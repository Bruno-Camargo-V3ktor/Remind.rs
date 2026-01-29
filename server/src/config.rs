use figment::{
    Figment,
    providers::{Env, Format, Json},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct ServerConfig {
    pub api_version: String,
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
pub struct SecurityConfig {
    pub users_key: String,
    pub reset_key: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct EmailConfig {
    pub smtp: String,
    pub from_name: String,
    pub from_email: String,
    pub app_key: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ConfigApp {
    pub server: ServerConfig,
    pub security: SecurityConfig,
    pub email: EmailConfig,
    pub surreal_db: Option<SurrealDbConfig>,
}

pub fn load_config() -> ConfigApp {
    Figment::new()
        .merge(Env::prefixed("REMIND_").split("__"))
        .merge(Json::file("config.json"))
        .extract::<ConfigApp>()
        .expect("Invalid Configuration")
}
