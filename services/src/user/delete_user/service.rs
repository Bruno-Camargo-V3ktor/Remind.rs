use super::DeleteUserError;
use crate::{Service, user::UserRepositoryType};
use domain::models::UserId;
use repository::RepositoryError;

pub struct DeleteUserService {
    pub user_repo: UserRepositoryType,
}

#[async_trait::async_trait]
impl Service for DeleteUserService {
    type Args = UserId;
    type Out = ();

    async fn run(&self, args: Self::Args) -> Result<Self::Out, DeleteUserError> {
        let user_id = args;

        let user = self.user_repo.delete(user_id).await;

        match user {
            Ok(_) => Ok(()),

            Err(e) => match e {
                RepositoryError::EntityNotFound(_) => {
                    return Err(DeleteUserError::UserNotExist);
                }
                _ => return Err(DeleteUserError::RepositoryError(e.to_string())),
            },
        }
    }
}
