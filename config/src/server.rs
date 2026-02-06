use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct ServerConfig {
    pub api_version: String,
    pub addr: String,
    pub hostname: String,
    pub workers: usize,
    pub keep_alive_secs: u64,
}
