use crate::{Repository, RepositoryResult};
use domain::models::{User, UserId};

pub type UserEntity = User;

#[async_trait::async_trait]
pub trait UserRepository: Repository<Entity = UserEntity, Id = UserId> {
    async fn get_by_email(&self, email: String) -> RepositoryResult<UserEntity>;
}
