use domain::models::Note;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::is_valid;

#[derive(Validate, Serialize, Deserialize)]
pub struct CreateNoteDTO {
    #[validate(length(min = 1, max = 50))]
    pub title: String,

    pub image: Option<String>,

    pub color: Option<u32>,
}

impl CreateNoteDTO {
    pub fn to_note(&self) -> Result<Note, Vec<String>> {
        match is_valid(self) {
            Ok(_) => Ok(Note::new(&self.title, &self.color)),

            Err(err) => Err(err),
        }
    }
}
