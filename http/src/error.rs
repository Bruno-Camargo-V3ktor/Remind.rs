use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ErrorInfos {
    code: String,
    message: String,
    layer: String,
}

impl ErrorInfos {
    pub fn new(code: String, message: String, layer: String) -> Self {
        Self {
            code,
            message,
            layer,
        }
    }
}
