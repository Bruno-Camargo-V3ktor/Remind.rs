use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct UserId(Uuid);

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub email: String,
    pub password: String,
    pub bio: String,
    pub photo_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
