use crate::ServiceError;
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
#[serde(untagged)]
pub enum CreateUserError {
    #[error("email {0} already registered")]
    EmailRegistered(String),

    #[error("invalid field(s): {0:#?}")]
    FieldsError(Vec<String>),

    #[error("repository error, with message: {0}")]
    RepositoryError(String),

    #[error("unknow create user error")]
    Unknown,
}

impl ServiceError for CreateUserError {
    fn code(&self) -> String {
        match self {
            Self::EmailRegistered(_) => "EMAIL_ALREADY_EXISTS".into(),
            Self::FieldsError(_) => "INVALID_FIELDS".into(),
            Self::RepositoryError(_) => "DATABASE_ERROR".into(),
            Self::Unknown => "INTERNAL_SERVER_ERROR".into(),
        }
    }

    fn content(&self) -> &impl serde::Serialize {
        self
    }

    fn description(&self) -> String {
        self.to_string()
    }
}
