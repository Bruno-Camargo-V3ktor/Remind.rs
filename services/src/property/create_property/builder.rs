use crate::{CreatePropertyService, ServiceBuilder};

use super::super::PropertyRepositoryType;

pub struct CreatePropertyBuilder {
    repo_property: Option<PropertyRepositoryType>,
}

impl CreatePropertyBuilder {
    pub fn repo_property(mut self, repo: PropertyRepositoryType) -> Self {
        self.repo_property = Some(repo);
        self
    }
}

impl ServiceBuilder for CreatePropertyBuilder {
    type S = CreatePropertyService;

    fn new() -> Self {
        Self {
            repo_property: None,
        }
    }

    fn build(self) -> Self::S {
        Self::S {
            property_repo: self.repo_property.expect(""),
        }
    }
}
