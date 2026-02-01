use super::{FileAction, FileServiceError};
use crate::Service;
use std::fs::File;

pub struct FileService {}

#[async_trait::async_trait]
impl Service for FileService {
    type Args = FileAction;
    type Out = Option<File>;

    async fn run(&self, args: Self::Args) -> Result<Self::Out, FileServiceError> {
        match args {
            FileAction::Save { file, dst } => {
                todo!()
            }

            FileAction::Open { path } => {
                todo!()
            }

            FileAction::Move { src, dst, copy } => {
                todo!()
            }

            FileAction::Delete { src } => {
                todo!()
            }
        }
    }
}
