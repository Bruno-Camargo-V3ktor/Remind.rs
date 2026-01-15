use domain::models::{Property, PropertyTypes};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::is_valid;

#[derive(Validate, Serialize, Deserialize)]
pub struct CreatePropertyDTO {
    #[validate(length(min = 1, max = 50))]
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: PropertyTypes,
    pub value: Option<u32>,
    pub color: u32,
}

impl CreatePropertyDTO {
    pub fn to_property(&self) -> Result<Property, Vec<String>> {
        match is_valid(self) {
            Ok(_) => Ok(Property::new(
                &self.r#type,
                &self.name,
                self.color,
                self.value.unwrap_or(0),
            )),

            Err(err) => Err(err),
        }
    }
}
