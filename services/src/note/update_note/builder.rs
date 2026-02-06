use crate::{ServiceBuilder, UpdateNoteService};

use super::super::NoteRepositoryType;

pub struct UpdateNoteBuilder {
    note_repo: Option<NoteRepositoryType>,
}

impl UpdateNoteBuilder {
    pub fn note_repo(mut self, repo: NoteRepositoryType) -> Self {
        self.note_repo = Some(repo);
        self
    }
}

impl ServiceBuilder for UpdateNoteBuilder {
    type S = UpdateNoteService;

    fn new() -> Self {
        Self { note_repo: None }
    }

    fn build(self) -> Self::S {
        Self::S {
            note_repo: self.note_repo.expect(""),
        }
    }
}
