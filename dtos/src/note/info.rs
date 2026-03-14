use chrono::{DateTime, Utc};
use domain::models::{Note, NoteId, PropertyId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct NoteInfoDTO {
    pub id: NoteId,
    pub title: String,
    pub content: String,
    pub color: u32,
    pub propertys: Vec<PropertyId>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl NoteInfoDTO {
    pub fn from_note(note: (Note, Vec<PropertyId>)) -> Self {
        let (note, properties) = note;

        Self {
            id: note.id,
            title: note.title,
            content: note.content,
            color: note.color,
            propertys: properties,
            created_at: note.created_at,
            updated_at: note.updated_at,
        }
    }
}
