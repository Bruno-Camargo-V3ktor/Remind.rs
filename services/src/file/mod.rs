use std::fs::File;

mod service;
use serde::Serialize;
pub use service::*;
use thiserror::Error;

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
        file: File,
        dst: String,
    },
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
