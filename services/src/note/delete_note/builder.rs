use crate::{DeletePropertyService, ServiceBuilder};

use super::super::PropertyRepositoryType;

pub struct DeletePropertyBuilder {
    repo_property: Option<PropertyRepositoryType>,
}

impl DeletePropertyBuilder {
    pub fn repo_property(mut self, repo: PropertyRepositoryType) -> Self {
        self.repo_property = Some(repo);
        self
    }
}

impl ServiceBuilder for DeletePropertyBuilder {
    type S = DeletePropertyService;

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
