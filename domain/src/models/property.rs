use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct PropertyId(Uuid);

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct Property {
    pub id: PropertyId,
    #[serde(rename = "type")]
    pub types: PropertyTypes,
    pub name: String,
    pub color: u32,
    pub value: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]

pub enum PropertyTypes {
    Category,
    Priority,
}
