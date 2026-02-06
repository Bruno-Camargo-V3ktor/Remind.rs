use crate::{LocalStorageService, ServiceBuilder};
use std::path::PathBuf;

pub struct LocalStorageBuilder {
    base: Option<PathBuf>,
}

impl LocalStorageBuilder {
    pub fn base(mut self, base: impl Into<PathBuf>) -> Self {
        self.base = Some(base.into());
        self
    }
}

impl ServiceBuilder for LocalStorageBuilder {
    type S = LocalStorageService;

    fn new() -> Self {
        Self { base: None }
    }

    fn build(self) -> Self::S {
        LocalStorageService {
            base: self.base.expect(""),
        }
    }
}
