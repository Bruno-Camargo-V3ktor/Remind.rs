use serde::{Deserialize, Serialize};

use crate::{
    EmailConfig, LocalStorgeConfig, S3StorageConfig, SecurityConfig, ServerConfig, SurrealDbConfig,
};

#[derive(Deserialize, Serialize, Clone)]
pub struct ConfigApp {
    pub server: ServerConfig,
    pub security: SecurityConfig,
    pub email: EmailConfig,
    pub surreal_db: Option<SurrealDbConfig>,
    pub s3_storage: Option<S3StorageConfig>,
    pub local_storage: LocalStorgeConfig,
}
