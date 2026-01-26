use actix_web::HttpResponse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    success: bool,
    data: Option<Value>,
    meta: Option<MetaInfos>,
    error: Option<Error>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MetaInfos {
    timestamp: DateTime<Utc>,
    version: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Error {
    code: String,
    message: String,
    layer: String,
}
