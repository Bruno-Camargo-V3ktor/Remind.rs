use crate::ServiceError;
use repository::property::PropertyRepository;
use serde::Serialize;
use std::sync::Arc;
use thiserror::Error;

mod create_property;
mod delete_property;
pub use create_property::*;
pub use delete_property::*;

pub type PropertyRepositoryType = Arc<dyn PropertyRepository + 'static + Send + Sync>;

#[derive(Error, Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum PropertysServiceErrors {
    #[error("Property already exists with name {0}")]
    PropertyAlreadyExists(String),

    #[error("invalid field(s): {0:#?}")]
    FieldsError(Vec<String>),

    #[error("repository error, with message: {0}")]
    RepositoryError(String),

    #[error("property not found")]
    PropertyNotExist,
}

impl ServiceError for PropertysServiceErrors {
    fn code(&self) -> String {
        match self {
            Self::PropertyAlreadyExists(_) => "PROPERTY_ALREADY_EXISTS".into(),
            Self::FieldsError(_) => "INVALID_FIELDS".into(),
            Self::RepositoryError(_) => "DATABASE_ERROR".into(),
            Self::PropertyNotExist => "PROPERTY_NOT_EXIST".into(),
        }
    }

    fn content(&self) -> &impl serde::Serialize {
        self
    }

    fn description(&self) -> String {
        self.to_string().trim().to_string()
    }
}
