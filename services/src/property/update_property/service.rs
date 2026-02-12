use super::super::PropertyRepositoryType;
use crate::{PropertysServiceErrors, Service};
use domain::models::{Property, PropertyId, UserId};
use dtos::UpdatePropertyDTO;
use repository::RepositoryError;

pub struct UpdatePropertyService {
    pub property_repo: PropertyRepositoryType,
}

#[async_trait::async_trait]
impl Service for UpdatePropertyService {
    type Args = (UserId, PropertyId, UpdatePropertyDTO);
    type Out = Property;

    async fn run(&self, args: Self::Args) -> Result<Self::Out, PropertysServiceErrors> {
        let (user_id, property_id, dto) = args;

        if let Some(name) = &dto.name {
            let is_existed = self
                .property_repo
                .get_by_name(user_id.clone(), name.clone())
                .await
                .is_ok();

            if is_existed {
                return Err(PropertysServiceErrors::PropertyAlreadyExists(name.clone()));
            }
        }

        let new_property = match self.property_repo.get_by_id(property_id).await {
            Ok(property) => match dto.to_property(&property.0) {
                Ok(p) => p,
                Err(e) => return Err(PropertysServiceErrors::FieldsError(e)),
            },

            Err(err) => match err {
                RepositoryError::EntityNotFound(_) => {
                    return Err(PropertysServiceErrors::PropertyNotExist);
                }
                _ => return Err(PropertysServiceErrors::RepositoryError(err.to_string())),
            },
        };

        match self.property_repo.update((new_property, user_id)).await {
            Ok(entity) => return Ok(entity.0),

            Err(err) => match err {
                RepositoryError::EntityNotFound(_) => {
                    return Err(PropertysServiceErrors::PropertyNotExist);
                }
                _ => return Err(PropertysServiceErrors::RepositoryError(err.to_string())),
            },
        }
    }
}
