use domain::models::{Note, UserId};
use repository::RepositoryError;

use crate::{NoteServiceErrors, Service};

use super::super::NoteRepositoryType;

pub struct ListNoteService {
    pub note_repo: NoteRepositoryType,
}

#[async_trait::async_trait]
impl Service for ListNoteService {
    type Args = UserId;
    type Out = Vec<Note>;

    async fn run(&self, args: Self::Args) -> Result<Self::Out, NoteServiceErrors> {
        let user_id = args;

        match self.note_repo.list_all_by_user(user_id).await {
            Ok(list) => return Ok(list.into_iter().map(|(n, _, _)| n).collect()),

            Err(err) => match err {
                RepositoryError::EntityNotFound(_) => {
                    return Err(NoteServiceErrors::NoteNotExist);
                }
                _ => return Err(NoteServiceErrors::RepositoryError(err.to_string())),
            },
        }
    }
}
