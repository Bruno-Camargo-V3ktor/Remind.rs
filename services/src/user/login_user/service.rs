use super::LoginUserError;
use crate::{Service, user::UserRepositoryType};
use domain::models::UserId;
use repository::RepositoryError;
use security::hash::PasswordHash;
use std::sync::Arc;

pub struct LoginUserService {
    pub user_repo: UserRepositoryType,
    pub password_hash: Arc<dyn PasswordHash + Send + Sync + 'static>,
}

#[async_trait::async_trait]
impl Service for LoginUserService {
    type Args = (String, String);
    type Out = UserId;

    async fn run(&self, args: Self::Args) -> Result<Self::Out, LoginUserError> {
        let (email, password) = args;

        let user = self.user_repo.get_by_email(email.clone()).await;

        match user {
            Ok(user) => {
                let hash_pass = user.password.clone();
                let is_valid = self.password_hash.validate(&password, &hash_pass);

                if is_valid {
                    return Ok(user.id);
                } else {
                    return Err(LoginUserError::InvalidCredentials);
                }
            }

            Err(e) => match e {
                RepositoryError::EntityNotFound(_) => {
                    return Err(LoginUserError::UserNotExist);
                }
                _ => return Err(LoginUserError::RepositoryError(e.to_string())),
            },
        }
    }
}
