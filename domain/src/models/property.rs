use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct PropertyId(Uuid);

impl Default for PropertyId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct Property {
    pub id: PropertyId,
    #[serde(rename = "type")]
    pub r#type: PropertyTypes,
    pub name: String,
    pub color: u32,
    pub value: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]

pub enum PropertyTypes {
    Category,
    Priority,
}

impl Property {
    pub fn new(r#type: &PropertyTypes, name: &str, color: u32, value: u32) -> Self {
        Self {
            id: PropertyId::default(),
            r#type: r#type.clone(),
            name: name.to_owned(),
            color,
            value,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
