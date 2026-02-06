use chrono::{DateTime, Utc};
use domain::models::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct InfoUserDTO {
    pub name: String,
    pub email: String,
    pub bio: String,
    pub photo_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl InfoUserDTO {
    pub fn from_user(user: &User) -> Self {
        Self {
            name: user.name.clone(),
            email: user.email.clone(),
            bio: user.bio.clone(),
            photo_url: user.photo_url.clone(),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
