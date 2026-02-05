use domain::models::PropertyId;
use repository::RepositoryError;

use crate::{PropertysServiceErrors, Service};

use super::super::PropertyRepositoryType;

pub struct DeletePropertyService {
    pub property_repo: PropertyRepositoryType,
}

#[async_trait::async_trait]
impl Service for DeletePropertyService {
    type Args = PropertyId;
    type Out = ();

    async fn run(&self, args: Self::Args) -> Result<Self::Out, PropertysServiceErrors> {
        let property_id = args;

        match self.property_repo.delete(property_id).await {
            Ok(_) => return Ok(()),

            Err(err) => match err {
                RepositoryError::EntityNotFound(_) => {
                    return Err(PropertysServiceErrors::PropertyNotExist);
                }
                _ => return Err(PropertysServiceErrors::RepositoryError(err.to_string())),
            },
        }
    }
}
