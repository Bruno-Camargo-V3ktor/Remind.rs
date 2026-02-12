use super::super::NoteRepositoryType;
use crate::{NoteServiceErrors, Service};
use domain::models::{Note, NoteId, UserId};
use dtos::UpdateNoteDTO;
use repository::RepositoryError;

pub struct UpdateNoteService {
    pub note_repo: NoteRepositoryType,
}

#[async_trait::async_trait]
impl Service for UpdateNoteService {
    type Args = (UserId, NoteId, UpdateNoteDTO);
    type Out = Note;

    async fn run(&self, args: Self::Args) -> Result<Self::Out, NoteServiceErrors> {
        let (user_id, note_id, dto) = args;
        let propertys = dto.propertys.clone();

        if let Some(title) = &dto.title {
            let is_existed = self
                .note_repo
                .get_by_title(user_id.clone(), title.clone())
                .await
                .is_ok();

            if is_existed {
                return Err(NoteServiceErrors::NoteAlreadyExists(title.clone()));
            }
        }

        let mut new_property = match self.note_repo.get_by_id(note_id).await {
            Ok(property) => match dto.to_note(&property.0) {
                Ok(p) => (p, property.1, property.2),
                Err(e) => return Err(NoteServiceErrors::FieldsError(e)),
            },

            Err(err) => match err {
                RepositoryError::EntityNotFound(_) => {
                    return Err(NoteServiceErrors::NoteNotExist);
                }
                _ => return Err(NoteServiceErrors::RepositoryError(err.to_string())),
            },
        };

        if let Some(propertys) = propertys {
            new_property.2 = propertys;
        }

        match self.note_repo.update(new_property).await {
            Ok(entity) => return Ok(entity.0),

            Err(err) => match err {
                RepositoryError::EntityNotFound(_) => {
                    return Err(NoteServiceErrors::NoteNotExist);
                }
                _ => return Err(NoteServiceErrors::RepositoryError(err.to_string())),
            },
        }
    }
}
