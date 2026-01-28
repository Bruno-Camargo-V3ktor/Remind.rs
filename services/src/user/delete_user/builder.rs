use super::DeleteUserService;
use crate::{ServiceBuilder, user::UserRepositoryType};

pub struct DeleteUserBuilder {
    user_repository: Option<UserRepositoryType>,
}

impl DeleteUserBuilder {
    pub fn user_repository(mut self, repo: UserRepositoryType) -> Self {
        self.user_repository = Some(repo);
        self
    }
}

impl ServiceBuilder for DeleteUserBuilder {
    type S = DeleteUserService;

    fn new() -> Self {
        Self {
            user_repository: None,
        }
    }

    fn build(self) -> Self::S {
        DeleteUserService {
            user_repo: self.user_repository.expect(""),
        }
    }
}
