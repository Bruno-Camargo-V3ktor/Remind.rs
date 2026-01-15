use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Serialize, Deserialize)]
pub struct ResetPasswordDTO {
    #[validate(length(min = 1, max = 20))]
    pub new_password: String,

    pub validate_token: String,
}

impl ResetPasswordDTO {}
