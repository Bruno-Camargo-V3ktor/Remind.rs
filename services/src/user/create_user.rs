use crate::{Service, ServiceBuilder, ServiceError, user::UserRepositoryType};
use domain::models::User;
use dtos::CreateUserDTO;
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

    async fn run(&self) -> Result<Self::Out, CreateUserError> {
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
#[derive(Error, Debug)]
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

impl ServiceError for CreateUserError {}
