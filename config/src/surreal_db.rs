use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct SurrealDbConfig {
    pub url: String,
    pub username: String,
    pub password: String,
    pub namespace: String,
    pub database: String,
}
