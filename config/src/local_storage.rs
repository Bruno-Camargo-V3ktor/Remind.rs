use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct LocalStorgeConfig {
    pub storage_dir: String,
    pub public_dir: Option<String>,
}
