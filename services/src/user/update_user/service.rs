use super::super::UserServiceErrors;
use crate::{Service, user::UserRepositoryType};
use domain::models::{User, UserId};
use dtos::UpdateUserDTO;
use repository::RepositoryError;
use security::hash::PasswordHash;
use std::sync::Arc;

pub struct UpdateUserService {
    pub user_repo: UserRepositoryType,
    pub password_hash: Arc<dyn PasswordHash + Send + Sync + 'static>,
}

#[async_trait::async_trait]
impl Service for UpdateUserService {
    type Args = (UserId, UpdateUserDTO);
    type Out = User;

    async fn run(&self, args: Self::Args) -> Result<Self::Out, UserServiceErrors> {
        let (user_id, mut dto) = args;

        let user = self.user_repo.get_by_id(user_id.clone()).await;

        match user {
            Ok(mut user) => {
                if let Some(new_email) = &dto.email {
                    let res = self.user_repo.get_by_email(new_email.clone()).await;
                    if res.is_ok() {
                        return Err(UserServiceErrors::EmailRegistered(new_email.clone()));
                    }
                }

                if let Some(new_password) = dto.password.take() {
                    dto.password = Some(self.password_hash.generate(&new_password));
                }

                merge_user(&mut user, &mut dto);

                let res = self
                    .user_repo
                    .update(user)
                    .await
                    .map_err(|e| UserServiceErrors::RepositoryError(e.to_string()));

                return res;
            }

            Err(e) => match e {
                RepositoryError::EntityNotFound(_) => {
                    return Err(UserServiceErrors::UserNotExist);
                }
                _ => return Err(UserServiceErrors::RepositoryError(e.to_string())),
            },
        }
    }
}

fn merge_user(user: &mut User, dto: &mut UpdateUserDTO) {
    user.name = dto.name.clone().unwrap_or(user.name.clone());
    user.email = dto.email.clone().unwrap_or(user.email.clone());
    user.password = dto.password.clone().unwrap_or(user.password.clone());
    user.bio = dto.bio.clone().unwrap_or(user.bio.clone());
    user.photo_url = dto.photo.clone().unwrap_or(user.photo_url.clone());
}
