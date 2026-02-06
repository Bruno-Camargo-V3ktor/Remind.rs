use domain::models::{Note, PropertyId};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::is_valid;

#[derive(Validate, Serialize, Deserialize)]
pub struct UpdateNoteDTO {
    #[validate(length(min = 3, max = 50))]
    pub title: Option<String>,
    pub content: Option<String>,
    pub color: Option<u32>,
    pub propertys: Option<Vec<PropertyId>>,
}

impl UpdateNoteDTO {
    pub fn to_note(self, old: &Note) -> Result<Note, Vec<String>> {
        match is_valid(&self) {
            Ok(_) => Ok(Note {
                id: old.id.clone(),

                title: self.title.unwrap_or(old.title.clone()),
                content: self.content.unwrap_or(old.content.clone()),
                color: self.color.unwrap_or(old.color),

                created_at: old.created_at,
                updated_at: old.updated_at,
            }),

            Err(err) => Err(err),
        }
    }
}
