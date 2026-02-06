use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct S3StorageConfig {
    pub url: String,
    pub access_key_id: String,
    pub access_key_secret: String,
    pub provide: String,
    pub region: String,
    pub temp_files_path: String,
}
