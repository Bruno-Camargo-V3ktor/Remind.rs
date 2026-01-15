use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Serialize, Deserialize)]
pub struct UpdateUserDTO {
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,

    #[validate(email)]
    pub email: Option<String>,

    #[validate(length(min = 10, max = 500))]
    pub bio: Option<String>,

    pub photo: Option<String>,
}

impl UpdateUserDTO {}
