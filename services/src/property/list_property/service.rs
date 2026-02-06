use domain::models::{Property, UserId};
use repository::RepositoryError;

use crate::{PropertysServiceErrors, Service};

use super::super::PropertyRepositoryType;

pub struct ListPropertyService {
    pub property_repo: PropertyRepositoryType,
}

#[async_trait::async_trait]
impl Service for ListPropertyService {
    type Args = UserId;
    type Out = Vec<Property>;

    async fn run(&self, args: Self::Args) -> Result<Self::Out, PropertysServiceErrors> {
        let user_id = args;

        match self.property_repo.list_all_by_user(user_id).await {
            Ok(list) => return Ok(list.into_iter().map(|(p, _)| p).collect()),

            Err(err) => match err {
                RepositoryError::EntityNotFound(_) => {
                    return Err(PropertysServiceErrors::PropertyNotExist);
                }
                _ => return Err(PropertysServiceErrors::RepositoryError(err.to_string())),
            },
        }
    }
}
