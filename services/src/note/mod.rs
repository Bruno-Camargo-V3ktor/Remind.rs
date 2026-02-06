use crate::ServiceError;
use repository::note::NoteRepository;
use serde::Serialize;
use std::sync::Arc;
use thiserror::Error;

mod create_note;
mod delete_note;
//mod update_note;

pub use create_note::*;
pub use delete_note::*;
//pub use update_note::*;

pub type NoteRepositoryType = Arc<dyn NoteRepository + 'static + Send + Sync>;

#[derive(Error, Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum NoteServiceErrors {
    #[error("Property already exists with name {0}")]
    NoteAlreadyExists(String),

    #[error("invalid field(s): {0:#?}")]
    FieldsError(Vec<String>),

    #[error("repository error, with message: {0}")]
    RepositoryError(String),

    #[error("property not found")]
    NoteNotExist,

    #[error("unknow property service error")]
    Unknown,
}

impl ServiceError for NoteServiceErrors {
    fn code(&self) -> String {
        match self {
            Self::NoteAlreadyExists(_) => "NOTE_ALREADY_EXISTS".into(),
            Self::FieldsError(_) => "INVALID_FIELDS".into(),
            Self::RepositoryError(_) => "DATABASE_ERROR".into(),
            Self::NoteNotExist => "NOTE_NOT_EXIST".into(),
            Self::Unknown => "INTERNAL_SERVER_ERROR".into(),
        }
    }

    fn content(&self) -> &impl serde::Serialize {
        self
    }

    fn description(&self) -> String {
        self.to_string().trim().to_string()
    }
}
