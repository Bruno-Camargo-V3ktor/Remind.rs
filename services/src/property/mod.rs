use repository::property::PropertyRepository;
use serde::Serialize;
use std::sync::Arc;
use thiserror::Error;

mod create_property;
pub use create_property::*;

use crate::ServiceError;

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
}

impl ServiceError for PropertysServiceErrors {
    fn code(&self) -> String {
        match self {
            Self::PropertyAlreadyExists(_) => "PROPERTY_ALREADY_EXISTS".into(),
            Self::FieldsError(_) => "INVALID_FIELDS".into(),
            Self::RepositoryError(_) => "DATABASE_ERROR".into(),
        }
    }

    fn content(&self) -> &impl serde::Serialize {
        self
    }

    fn description(&self) -> String {
        self.to_string().trim().to_string()
    }
}
