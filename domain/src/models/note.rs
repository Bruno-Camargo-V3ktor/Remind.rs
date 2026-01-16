use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct NoteId(pub Uuid);

impl Default for NoteId {
    fn default() -> Self {
        NoteId(Uuid::new_v4())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct Note {
    pub id: NoteId,
    pub title: String,
    pub content: String,
    pub color: Option<u32>,
    pub image: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Note {
    pub fn new(title: impl Into<String>, image: &Option<String>, color: &Option<u32>) -> Self {
        Self {
            id: NoteId::default(),
            title: title.into(),
            content: String::new(),
            color: *color,
            image: image.clone(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
