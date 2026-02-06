use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct UserId(pub Uuid);

impl Default for UserId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl FromStr for UserId {
    type Err = ();
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        let uuid = Uuid::from_str(id).map_err(|_| ())?;
        Ok(UserId(uuid))
    }
}

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

impl User {
    pub fn new(name: String, email: String, password: String) -> Self {
        Self {
            id: UserId::default(),
            name,
            email,
            password,
            bio: String::new(),
            photo_url: "default.png".into(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
