use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HttpResponse {
    success: bool,
    data: Option<Value>,
    meta: Option<MetaInfos>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MetaInfos {
    timestamp: DateTime<Utc>,
    version: String,
}
