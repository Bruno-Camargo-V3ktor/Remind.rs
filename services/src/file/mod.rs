use serde::Serialize;
use thiserror::Error;

mod local_storage;
mod s3_storage;
pub use local_storage::*;
pub use s3_storage::*;

use crate::ServiceError;

pub enum FileAction {
    Open {
        path: String,
    },
    Move {
        src: String,
        dst: String,
        copy: bool,
    },
    Delete {
        src: String,
    },
    Save {
        bytes: Vec<u8>,
        dst: String,
    },
}

fn content_type(key: &str) -> &str {
    let extension = key.split('.').last().unwrap_or("");

    match extension.to_lowercase().as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "webp" => "image/webp",
        "pdf" => "application/pdf",
        "json" => "application/json",
        "txt" => "text/plain",
        "html" => "text/html",
        "mp4" => "video/mp4",
        "wav" => "audio/wav",
        "mp3" => "audio/mpeg",
        _ => "application/octet-stream",
    }
}

#[derive(Error, Debug, Serialize)]
#[serde(untagged)]
pub enum FileServiceError {
    #[error("file service error: {0}")]
    Error(String),
}

impl ServiceError for FileServiceError {
    fn code(&self) -> String {
        match self {
            Self::Error(_) => "IO_SERVER_ERROR".into(),
        }
    }

    fn content(&self) -> &impl serde::Serialize {
        self
    }

    fn description(&self) -> String {
        self.to_string().trim().to_string()
    }
}
