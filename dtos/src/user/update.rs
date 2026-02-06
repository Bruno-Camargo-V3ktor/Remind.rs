use domain::models::User;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::is_valid;

#[derive(Validate, Serialize, Deserialize)]
pub struct UpdateUserDTO {
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,

    #[validate(email)]
    pub email: Option<String>,

    #[validate(length(min = 10, max = 500))]
    pub bio: Option<String>,

    #[validate(length(min = 1, max = 20))]
    pub password: Option<String>,

    pub photo: Option<String>,
}

impl UpdateUserDTO {
    pub fn to_user(&self, old: &User) -> Result<User, Vec<String>> {
        match is_valid(self) {
            Ok(_) => {
                let mut user = old.clone();
                user.name = self.name.clone().unwrap_or(user.name.clone());
                user.email = self.email.clone().unwrap_or(user.email.clone());
                user.password = self.password.clone().unwrap_or(user.password.clone());
                user.bio = self.bio.clone().unwrap_or(user.bio.clone());
                user.photo_url = self.photo.clone().unwrap_or(user.photo_url.clone());

                Ok(user)
            }

            Err(err) => Err(err),
        }
    }
}
