use super::super::UserServiceErrors;
use crate::{Service, user::UserRepositoryType};
use domain::models::User;
use dtos::CreateUserDTO;
use security::hash::PasswordHash;
use std::sync::Arc;

pub struct CreateUserService {
    pub user_repo: UserRepositoryType,
    pub password_hash: Arc<dyn PasswordHash + Send + Sync + 'static>,
}

#[async_trait::async_trait]
impl Service for CreateUserService {
    type Args = CreateUserDTO;
    type Out = User;

    async fn run(&self, args: Self::Args) -> Result<Self::Out, UserServiceErrors> {
        let dto = args;

        let user_exist = self.user_repo.get_by_email(dto.email.clone()).await.is_ok();

        if user_exist {
            return Err(UserServiceErrors::EmailRegistered(dto.email.clone()));
        }

        match dto.to_user() {
            Ok(mut user) => {
                user.password = self.password_hash.generate(&user.password);

                match self.user_repo.create(user).await {
                    Ok(entity) => {
                        return Ok(entity);
                    }

                    Err(err) => return Err(UserServiceErrors::RepositoryError(err.to_string())),
                }
            }

            Err(field_erros) => return Err(UserServiceErrors::FieldsError(field_erros)),
        }
    }
}
