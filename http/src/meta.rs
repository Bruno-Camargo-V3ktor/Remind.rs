use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MetaInfos {
    timestamp: DateTime<Utc>,
    version: String,
}

impl MetaInfos {
    pub fn new(version: String) -> Self {
        Self {
            timestamp: Utc::now(),
            version,
        }
    }
}
