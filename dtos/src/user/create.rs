use domain::models::User;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::is_valid;

#[derive(Validate, Serialize, Deserialize)]
pub struct CreateUserDTO {
    #[validate(length(min = 1, max = 20))]
    pub first_name: String,

    #[validate(length(min = 1, max = 20))]
    pub last_name: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 1, max = 20))]
    pub password: String,
}

impl CreateUserDTO {
    pub fn to_user(&self) -> Result<User, Vec<String>> {
        match is_valid(self) {
            Ok(_) => Ok(User::new(
                format!("{} {}", self.first_name, self.last_name),
                self.email.clone(),
                self.password.clone(),
            )),

            Err(err) => Err(err),
        }
    }
}
