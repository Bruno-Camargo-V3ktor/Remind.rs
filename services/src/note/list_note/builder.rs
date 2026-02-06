use crate::{ListNoteService, ServiceBuilder};

use super::super::NoteRepositoryType;

pub struct ListNoteBuilder {
    note_repo: Option<NoteRepositoryType>,
}

impl ListNoteBuilder {
    pub fn note_repo(mut self, repo: NoteRepositoryType) -> Self {
        self.note_repo = Some(repo);
        self
    }
}

impl ServiceBuilder for ListNoteBuilder {
    type S = ListNoteService;

    fn new() -> Self {
        Self { note_repo: None }
    }

    fn build(self) -> Self::S {
        Self::S {
            note_repo: self.note_repo.expect(""),
        }
    }
}
