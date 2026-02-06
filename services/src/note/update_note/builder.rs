use crate::{ServiceBuilder, UpdatePropertyService};

use super::super::PropertyRepositoryType;

pub struct UpdatePropertyBuilder {
    repo_property: Option<PropertyRepositoryType>,
}

impl UpdatePropertyBuilder {
    pub fn repo_property(mut self, repo: PropertyRepositoryType) -> Self {
        self.repo_property = Some(repo);
        self
    }
}

impl ServiceBuilder for UpdatePropertyBuilder {
    type S = UpdatePropertyService;

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
