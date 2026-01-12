use crate::{Repository, RepositoryResult};
use domain::models::{Property, PropertyId, UserId};

#[async_trait::async_trait]
pub trait PropertyRepository: Repository<Entity = (Property, UserId), Id = PropertyId> {
    async fn list_all_by_user(&self, user_id: UserId) -> RepositoryResult<Vec<Self::Entity>>;
}
