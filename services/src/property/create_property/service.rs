use domain::models::UserId;
use dtos::CreatePropertyDTO;

use crate::{PropertysServiceErrors, Service};

use super::super::PropertyRepositoryType;

pub struct CreatePropertyService {
    pub property_repo: PropertyRepositoryType,
}

#[async_trait::async_trait]
impl Service for CreatePropertyService {
    type Args = (UserId, CreatePropertyDTO);
    type Out = ();

    async fn run(&self, args: Self::Args) -> Result<Self::Out, PropertysServiceErrors> {
        let (user_id, dto) = args;

        let is_existed_with_name = self
            .property_repo
            .list_all_by_user(user_id.clone())
            .await
            .unwrap_or_default()
            .iter()
            .find(|(p, _)| p.name == dto.name)
            .is_some();

        if is_existed_with_name {
            return Err(PropertysServiceErrors::PropertyAlreadyExists(dto.name));
        }

        Ok(())
    }
}
