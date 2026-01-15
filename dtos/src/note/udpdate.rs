use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Serialize, Deserialize)]
pub struct UpdateNoteDTO {
    #[validate(length(min = 3, max = 50))]
    pub title: Option<String>,

    pub content: String,

    pub image: Option<String>,

    pub color: Option<u32>,
}

impl UpdateNoteDTO {}
