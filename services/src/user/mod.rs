use crate::ServiceError;
use repository::user::UserRepository;
use serde::Serialize;
use std::sync::Arc;
use thiserror::Error;

mod create_user;
mod delete_user;
mod login_user;
mod update_user;

pub use create_user::*;
pub use delete_user::*;
pub use login_user::*;
pub use update_user::*;

pub type UserRepositoryType = Arc<dyn UserRepository + 'static + Send + Sync>;

#[derive(Error, Debug, Serialize)]
#[serde(untagged)]
pub enum UserServiceErrors {
    #[error("email {0} already registered")]
    EmailRegistered(String),

    #[error("invalid field(s): {0:#?}")]
    FieldsError(Vec<String>),

    #[error("repository error, with message: {0}")]
    RepositoryError(String),

    #[error("User not found registered")]
    UserNotExist,

    #[error("invalid credentials by User")]
    InvalidCredentials,

    #[error("unknow create user error")]
    Unknown,
}

impl ServiceError for UserServiceErrors {
    fn code(&self) -> String {
        match self {
            Self::EmailRegistered(_) => "EMAIL_ALREADY_EXISTS".into(),
            Self::FieldsError(_) => "INVALID_FIELDS".into(),
            Self::InvalidCredentials => "INVALID_CREDENTIALS".into(),
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
