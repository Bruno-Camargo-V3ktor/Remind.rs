use crate::{error::ErrorInfos, meta::MetaInfos};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Display;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Response {
    #[serde(skip, default)]
    pub status_code: u16,
    pub success: bool,
    pub data: Option<Value>,
    pub meta: Option<MetaInfos>,
    pub error: Option<ErrorInfos>,
}

impl Response {
    pub fn success(status_code: u16, data: &impl Serialize, version: &str) -> Self {
        Self {
            status_code,
            success: true,
            data: Some(serde_json::to_value(data).unwrap()),
            meta: Some(MetaInfos::new(version.to_owned())),
            error: None,
        }
    }

    pub fn error(
        status_code: u16,
        code: String,
        description: String,
        value: impl Serialize,
    ) -> Self {
        Self {
            status_code,
            success: false,
            data: None,
            meta: None,
            error: Some(ErrorInfos::new(
                code,
                description,
                serde_json::to_value(value).unwrap(),
            )),
        }
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

#[cfg(all(not(target_arch = "wasm32"), feature = "actix-web"))]
pub mod actix {
    use super::*;
    use actix_web::{
        HttpRequest, HttpResponse, Responder, ResponseError, body::BoxBody, http::StatusCode,
    };

    impl Responder for Response {
        type Body = BoxBody;
        fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
            self.error_response()
        }
    }

    impl ResponseError for Response {
        fn status_code(&self) -> StatusCode {
            StatusCode::from_u16(self.status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
        }

        fn error_response(&self) -> HttpResponse<BoxBody> {
            HttpResponse::build(self.status_code()).json(self)
        }
    }
}
