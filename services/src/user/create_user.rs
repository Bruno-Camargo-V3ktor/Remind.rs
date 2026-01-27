use crate::{Service, ServiceBuilder, ServiceError, user::UserRepositoryType};
use domain::models::User;
use dtos::CreateUserDTO;
use security::hash::PasswordHash;
use serde::Serialize;
use std::sync::Arc;
use thiserror::Error;

// Builder
pub struct CreateUserBuilder {
    user_repository: Option<UserRepositoryType>,
    password_hash: Option<Arc<dyn PasswordHash + Send + Sync + 'static>>,
}

impl CreateUserBuilder {
    pub fn user_repository(mut self, repo: UserRepositoryType) -> Self {
        self.user_repository = Some(repo);
        self
    }

    pub fn password_hash(
        mut self,
        password_hash: Arc<dyn PasswordHash + Send + Sync + 'static>,
    ) -> Self {
        self.password_hash = Some(password_hash);
        self
    }
}

impl ServiceBuilder for CreateUserBuilder {
    type S = CreateUserService;

    fn new() -> Self {
        Self {
            password_hash: None,
            user_repository: None,
        }
    }

    fn build(self) -> Self::S {
        CreateUserService {
            user_repo: self.user_repository.expect(""),
            password_hash: self.password_hash.expect(""),
        }
    }
}

// Service
pub struct CreateUserService {
    user_repo: UserRepositoryType,
    password_hash: Arc<dyn PasswordHash + Send + Sync + 'static>,
}

#[async_trait::async_trait]
impl Service for CreateUserService {
    type Args = CreateUserDTO;
    type Out = User;

    async fn run(&self, args: Self::Args) -> Result<Self::Out, CreateUserError> {
        let dto = args;

        let user_exist = self.user_repo.get_by_email(dto.email.clone()).await.is_ok();

        if user_exist {
            return Err(CreateUserError::EmailRegistered(dto.email.clone()));
        }

        match dto.to_user() {
            Ok(mut user) => {
                user.password = self.password_hash.generate(&user.password);

                match self.user_repo.create(user).await {
                    Ok(entity) => {
                        return Ok(entity);
                    }

                    Err(err) => return Err(CreateUserError::RepositoryError(err.to_string())),
                }
            }

            Err(field_erros) => return Err(CreateUserError::FieldsError(field_erros)),
        }
    }
}

//Error
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
