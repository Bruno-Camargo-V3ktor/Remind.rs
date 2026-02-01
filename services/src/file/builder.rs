use crate::{FileService, ServiceBuilder};
use std::path::PathBuf;

pub struct FileServiceBuilder {
    base: Option<PathBuf>,
}

impl FileServiceBuilder {
    pub fn base(mut self, base: impl Into<PathBuf>) -> Self {
        self.base = Some(base.into());
        self
    }
}

impl ServiceBuilder for FileServiceBuilder {
    type S = FileService;

    fn new() -> Self {
        Self { base: None }
    }

    fn build(self) -> Self::S {
        FileService {
            base: self.base.expect(""),
        }
    }
}
