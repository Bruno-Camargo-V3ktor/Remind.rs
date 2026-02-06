use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct SecurityConfig {
    pub users_key: String,
    pub reset_key: String,
}
