use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ServerConfig {
    pub addr: String,
    pub hostname: String,
    pub workers: usize,
    pub keep_alive_secs: u64,
}

#[derive(Deserialize, Serialize)]
pub struct SurrealDbConfig {
    pub url: String,
    pub username: String,
    pub password: String,
    pub namespace: String,
    pub database: String,
}

#[derive(Deserialize, Serialize)]
pub struct ConfigApp {
    pub server: ServerConfig,
    pub surreal_db: Option<SurrealDbConfig>,
}
