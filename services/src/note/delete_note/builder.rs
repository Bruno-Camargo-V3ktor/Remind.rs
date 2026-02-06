use crate::{DeleteNoteService, ServiceBuilder};

use super::super::NoteRepositoryType;

pub struct DeleteNoteBuilder {
    note_repo: Option<NoteRepositoryType>,
}

impl DeleteNoteBuilder {
    pub fn note_repo(mut self, repo: NoteRepositoryType) -> Self {
        self.note_repo = Some(repo);
        self
    }
}

impl ServiceBuilder for DeleteNoteBuilder {
    type S = DeleteNoteService;

    fn new() -> Self {
        Self { note_repo: None }
    }

    fn build(self) -> Self::S {
        Self::S {
            note_repo: self.note_repo.expect(""),
        }
    }
}
