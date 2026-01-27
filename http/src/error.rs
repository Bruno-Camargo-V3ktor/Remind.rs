use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ErrorInfos {
    code: String,
    description: String,
    value: Value,
}

impl ErrorInfos {
    pub fn new(code: String, description: String, payload: impl Serialize) -> Self {
        Self {
            code,
            description,
            value: serde_json::to_value(payload).unwrap(),
        }
    }
}
