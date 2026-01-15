use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Serialize, Deserialize)]
pub struct UpdatePropertyDTO {
    #[validate(length(min = 1, max = 50))]
    pub name: String,
    pub value: Option<u32>,
    pub color: u32,
}

impl UpdatePropertyDTO {}
