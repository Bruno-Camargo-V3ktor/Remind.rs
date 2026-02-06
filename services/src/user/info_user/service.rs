use super::super::UserServiceErrors;
use crate::{Service, user::UserRepositoryType};
use domain::models::UserId;
use dtos::InfoUserDTO;
use repository::RepositoryError;

pub struct InfoUserService {
    pub user_repo: UserRepositoryType,
}

#[async_trait::async_trait]
impl Service for InfoUserService {
    type Args = UserId;
    type Out = InfoUserDTO;

    async fn run(&self, args: Self::Args) -> Result<Self::Out, UserServiceErrors> {
        let user_id = args;

        let user = self.user_repo.get_by_id(user_id).await;

        match user {
            Ok(user) => Ok(InfoUserDTO::from_user(&user)),

            Err(e) => match e {
                RepositoryError::EntityNotFound(_) => {
                    return Err(UserServiceErrors::UserNotExist);
                }
                _ => return Err(UserServiceErrors::RepositoryError(e.to_string())),
            },
        }
    }
}
