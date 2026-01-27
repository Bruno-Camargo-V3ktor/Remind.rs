use super::CreateUserService;
use crate::{ServiceBuilder, user::UserRepositoryType};
use security::hash::PasswordHash;
use std::sync::Arc;

pub struct CreateUserBuilder {
    user_repository: Option<UserRepositoryType>,
    password_hash: Option<Arc<dyn PasswordHash + Send + Sync + 'static>>,
}

impl CreateUserBuilder {
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

impl ServiceBuilder for CreateUserBuilder {
    type S = CreateUserService;

    fn new() -> Self {
        Self {
            password_hash: None,
            user_repository: None,
        }
    }

    fn build(self) -> Self::S {
        CreateUserService {
            user_repo: self.user_repository.expect(""),
            password_hash: self.password_hash.expect(""),
        }
    }
}
