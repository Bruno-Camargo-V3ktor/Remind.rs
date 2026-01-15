use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Serialize, Deserialize)]
pub struct LoginUserDTO {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 1, max = 20))]
    pub password: String,
}

impl LoginUserDTO {}
