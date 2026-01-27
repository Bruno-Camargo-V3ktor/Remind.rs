use crate::ServiceError;
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
#[serde(untagged)]
pub enum LoginUserError {
    #[error("User by email not registered")]
    UserNotExist,

    #[error("invalid credentials by User")]
    InvalidCredentials,

    #[error("repository error, with message: {0}")]
    RepositoryError(String),

    #[error("unknow login user error")]
    Unknown,
}

impl ServiceError for LoginUserError {
    fn code(&self) -> String {
        match self {
            Self::UserNotExist => "USER_NOT_EXIST".into(),
            Self::RepositoryError(_) => "DATABASE_ERROR".into(),
            Self::InvalidCredentials => "INVALID_CREDENTIALS".into(),
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
