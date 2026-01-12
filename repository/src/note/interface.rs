use crate::{Repository, RepositoryResult};
use domain::models::{Note, NoteId, UserId};

#[async_trait::async_trait]
pub trait NoteRepository: Repository<Entity = (Note, UserId), Id = NoteId> {
    async fn list_all_by_user(&self, user_id: UserId) -> RepositoryResult<Vec<Self::Entity>>;
}
