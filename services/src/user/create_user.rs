use crate::{Service, ServiceBuilder, ServiceError, user::UserRepositoryType};
use domain::models::User;
use dtos::CreateUserDTO;
use serde::Serialize;
use std::any::TypeId;
use thiserror::Error;

// Builder
pub struct CreateUserBuilder {
    user_repository: UserRepositoryType,
}

impl CreateUserBuilder {
    pub fn build(&self, user: CreateUserDTO) -> CreateUserService {
        CreateUserService {
            user_repo: self.user_repository.clone(),
            create_user_dto: user,
        }
    }
}

impl ServiceBuilder for CreateUserBuilder {
    fn type_service(&self) -> std::any::TypeId {
        TypeId::of::<CreateUserService>()
    }
}

// Service
pub struct CreateUserService {
    user_repo: UserRepositoryType,
    create_user_dto: CreateUserDTO,
}

impl CreateUserService {
    pub fn builder(user_repo: UserRepositoryType) -> CreateUserBuilder {
        CreateUserBuilder {
            user_repository: user_repo,
        }
    }
}

#[async_trait::async_trait]
impl Service for CreateUserService {
    type Out = User;
    type Builder = CreateUserBuilder;

    async fn run(&self) -> Result<Self::Out, CreateUserError> {
        let user_exist = self
            .user_repo
            .get_by_email(self.create_user_dto.email.clone())
            .await
            .is_ok();

        if user_exist {
            return Err(CreateUserError::EmailRegistered(
                self.create_user_dto.email.clone(),
            ));
        }

        match self.create_user_dto.to_user() {
            Ok(user) => match self.user_repo.create(user).await {
                Ok(entity) => {
                    return Ok(entity);
                }

                Err(err) => return Err(CreateUserError::RepositoryError(err.to_string())),
            },

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
        format!("{self:?}")
    }
}
