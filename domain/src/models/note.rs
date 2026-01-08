use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NoteId(Uuid);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Note {
    pub id: NoteId,
    pub title: String,
    pub content: String,
    pub color: u32,
    pub image: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
