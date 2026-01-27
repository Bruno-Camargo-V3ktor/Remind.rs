use super::LoginUserService;
use crate::{ServiceBuilder, user::UserRepositoryType};
use security::hash::PasswordHash;
use std::sync::Arc;

pub struct LoginUserBuilder {
    user_repository: Option<UserRepositoryType>,
    password_hash: Option<Arc<dyn PasswordHash + Send + Sync + 'static>>,
}

impl LoginUserBuilder {
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

impl ServiceBuilder for LoginUserBuilder {
    type S = LoginUserService;

    fn new() -> Self {
        Self {
            password_hash: None,
            user_repository: None,
        }
    }

    fn build(self) -> Self::S {
        LoginUserService {
            user_repo: self.user_repository.expect(""),
            password_hash: self.password_hash.expect(""),
        }
    }
}
