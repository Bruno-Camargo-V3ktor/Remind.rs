use crate::{Repository, RepositoryResult};
use domain::models::{Property, PropertyId, UserId};

pub type PropertyEntity = (Property, UserId);

#[async_trait::async_trait]
pub trait PropertyRepository: Repository<Entity = PropertyEntity, Id = PropertyId> {
    async fn list_all_by_user(&self, user_id: UserId) -> RepositoryResult<Vec<Self::Entity>>;
}
