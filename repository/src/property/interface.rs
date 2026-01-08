use crate::{Repository, RepositoryResult};
use domain::models::UserId;

#[async_trait::async_trait]
pub trait PropertyRepository: Repository {
    async fn list_all_by_user(&self, user_id: UserId) -> RepositoryResult<Vec<Self::Entity>>;
}
