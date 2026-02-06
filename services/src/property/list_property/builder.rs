use crate::{ListPropertyService, ServiceBuilder};

use super::super::PropertyRepositoryType;

pub struct ListPropertyBuilder {
    repo_property: Option<PropertyRepositoryType>,
}

impl ListPropertyBuilder {
    pub fn repo_property(mut self, repo: PropertyRepositoryType) -> Self {
        self.repo_property = Some(repo);
        self
    }
}

impl ServiceBuilder for ListPropertyBuilder {
    type S = ListPropertyService;

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
