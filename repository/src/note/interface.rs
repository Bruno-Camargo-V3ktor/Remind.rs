use crate::{Repository, RepositoryResult};
use domain::models::{Note, NoteId, PropertyId, UserId};

pub type NoteEntity = (Note, UserId, Vec<PropertyId>);

#[async_trait::async_trait]
pub trait NoteRepository: Repository<Entity = NoteEntity, Id = NoteId> {
    async fn list_all_by_user(&self, user_id: UserId) -> RepositoryResult<Vec<Self::Entity>>;
}
