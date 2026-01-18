use std::str::FromStr;

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

impl FromStr for NoteId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(NoteId(Uuid::from_str(s).unwrap()))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct Note {
    pub id: NoteId,
    pub title: String,
    pub content: String,
    pub color: Option<u32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Note {
    pub fn new(title: impl Into<String>, color: &Option<u32>) -> Self {
        Self {
            id: NoteId::default(),
            title: title.into(),
            content: String::new(),
            color: *color,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
