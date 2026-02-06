use domain::models::NoteId;
use repository::RepositoryError;

use crate::{NoteServiceErrors, Service};

use super::super::NoteRepositoryType;

pub struct DeleteNoteService {
    pub note_repo: NoteRepositoryType,
}

#[async_trait::async_trait]
impl Service for DeleteNoteService {
    type Args = NoteId;
    type Out = ();

    async fn run(&self, args: Self::Args) -> Result<Self::Out, NoteServiceErrors> {
        let property_id = args;

        match self.note_repo.delete(property_id).await {
            Ok(_) => return Ok(()),

            Err(err) => match err {
                RepositoryError::EntityNotFound(_) => {
                    return Err(NoteServiceErrors::NoteNotExist);
                }
                _ => return Err(NoteServiceErrors::RepositoryError(err.to_string())),
            },
        }
    }
}
