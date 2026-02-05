use domain::models::{Property, UserId};
use dtos::CreatePropertyDTO;

use crate::{PropertysServiceErrors, Service};

use super::super::PropertyRepositoryType;

pub struct CreatePropertyService {
    pub property_repo: PropertyRepositoryType,
}

#[async_trait::async_trait]
impl Service for CreatePropertyService {
    type Args = (UserId, CreatePropertyDTO);
    type Out = Property;

    async fn run(&self, args: Self::Args) -> Result<Self::Out, PropertysServiceErrors> {
        let (user_id, dto) = args;

        let is_existed_with_name = self
            .property_repo
            .get_by_name(user_id.clone(), dto.name.clone())
            .await
            .is_ok();

        if is_existed_with_name {
            return Err(PropertysServiceErrors::PropertyAlreadyExists(dto.name));
        }

        match dto.to_property() {
            Ok(p) => match self.property_repo.create((p, user_id)).await {
                Ok(entity) => return Ok(entity.0),

                Err(err) => return Err(PropertysServiceErrors::RepositoryError(err.to_string())),
            },

            Err(fields_erros) => return Err(PropertysServiceErrors::FieldsError(fields_erros)),
        }
    }
}
