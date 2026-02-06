use super::super::NoteRepositoryType;
use crate::{NoteServiceErrors, Service};
use domain::models::{Note, UserId};
use dtos::CreateNoteDTO;

pub struct CreateNoteService {
    pub note_repo: NoteRepositoryType,
}

#[async_trait::async_trait]
impl Service for CreateNoteService {
    type Args = (UserId, CreateNoteDTO);
    type Out = Note;

    async fn run(&self, args: Self::Args) -> Result<Self::Out, NoteServiceErrors> {
        let (user_id, dto) = args;
        let propertys = dto.propertys.clone();

        let is_existed_with_title = self
            .note_repo
            .get_by_title(user_id.clone(), dto.title.clone())
            .await
            .is_ok();

        if is_existed_with_title {
            return Err(NoteServiceErrors::NoteAlreadyExists(dto.title));
        }

        match dto.to_note() {
            Ok(note) => match self.note_repo.create((note, user_id, propertys)).await {
                Ok(entity) => return Ok(entity.0),

                Err(err) => return Err(NoteServiceErrors::RepositoryError(err.to_string())),
            },

            Err(fields_erros) => return Err(NoteServiceErrors::FieldsError(fields_erros)),
        }
    }
}
