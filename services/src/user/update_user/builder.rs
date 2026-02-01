use super::UpdateUserService;
use crate::{ServiceBuilder, user::UserRepositoryType};
use security::hash::PasswordHash;
use std::sync::Arc;

pub struct UpdateUserBuilder {
    user_repository: Option<UserRepositoryType>,
    password_hash: Option<Arc<dyn PasswordHash + Send + Sync + 'static>>,
}

impl UpdateUserBuilder {
    pub fn user_repository(mut self, repo: UserRepositoryType) -> Self {
        self.user_repository = Some(repo);
        self
    }

    pub fn password_hash(mut self, hash: Arc<dyn PasswordHash + Send + Sync + 'static>) -> Self {
        self.password_hash = Some(hash);
        self
    }
}

impl ServiceBuilder for UpdateUserBuilder {
    type S = UpdateUserService;

    fn new() -> Self {
        Self {
            user_repository: None,
            password_hash: None,
        }
    }

    fn build(self) -> Self::S {
        UpdateUserService {
            user_repo: self.user_repository.expect(""),
            password_hash: self.password_hash.expect(""),
        }
    }
}
