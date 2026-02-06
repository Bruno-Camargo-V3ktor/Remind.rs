use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct EmailConfig {
    pub smtp: String,
    pub from_name: String,
    pub from_email: String,
    pub app_key: String,
}
