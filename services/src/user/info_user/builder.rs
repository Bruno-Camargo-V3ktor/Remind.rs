use super::InfoUserService;
use crate::{ServiceBuilder, user::UserRepositoryType};

pub struct InfoUserBuilder {
    user_repository: Option<UserRepositoryType>,
}

impl InfoUserBuilder {
    pub fn user_repository(mut self, repo: UserRepositoryType) -> Self {
        self.user_repository = Some(repo);
        self
    }
}

impl ServiceBuilder for InfoUserBuilder {
    type S = InfoUserService;

    fn new() -> Self {
        Self {
            user_repository: None,
        }
    }

    fn build(self) -> Self::S {
        InfoUserService {
            user_repo: self.user_repository.expect(""),
        }
    }
}
