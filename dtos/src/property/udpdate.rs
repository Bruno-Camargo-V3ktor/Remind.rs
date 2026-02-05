use domain::models::Property;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::is_valid;

#[derive(Validate, Serialize, Deserialize)]
pub struct UpdatePropertyDTO {
    #[validate(length(min = 1, max = 50))]
    pub name: Option<String>,
    pub value: Option<u32>,
    pub color: Option<u32>,
}

impl UpdatePropertyDTO {
    pub fn to_property(self, old: &Property) -> Result<Property, Vec<String>> {
        match is_valid(&self) {
            Ok(_) => Ok(Property {
                id: old.id.clone(),
                r#type: old.r#type.clone(),

                color: self.color.unwrap_or(old.color),
                name: self.name.unwrap_or(old.name.clone()),
                value: self.value.unwrap_or(old.value),

                created_at: old.created_at,
                updated_at: old.updated_at,
            }),

            Err(err) => Err(err),
        }
    }
}
