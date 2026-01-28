use crate::ServiceError;
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
#[serde(untagged)]
pub enum DeleteUserError {
    #[error("User not found registered")]
    UserNotExist,

    #[error("repository error, with message: {0}")]
    RepositoryError(String),

    #[error("unknow login user error")]
    Unknown,
}

impl ServiceError for DeleteUserError {
    fn code(&self) -> String {
        match self {
            Self::UserNotExist => "USER_NOT_EXIST".into(),
            Self::RepositoryError(_) => "DATABASE_ERROR".into(),
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
