use super::super::{FileAction, FileServiceError};
use crate::Service;
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};

pub struct LocalStorageService {
    pub base: PathBuf,
}

#[async_trait::async_trait]
impl Service for LocalStorageService {
    type Args = FileAction;
    type Out = Option<File>;

    async fn run(&self, args: Self::Args) -> Result<Self::Out, FileServiceError> {
        let mut path_file = self.base.clone();

        match args {
            FileAction::Save { bytes, dst } => {
                path_file.push(&dst);
                let mut new_file = File::create_new(path_file)
                    .map_err(|e| FileServiceError::Error(e.to_string()))?;

                new_file
                    .write_all(&bytes)
                    .map_err(|e| FileServiceError::Error(e.to_string()))?;

                Ok(None)
            }

            FileAction::Open { path } => {
                path_file.push(&path);
                let file =
                    File::open(path_file).map_err(|e| FileServiceError::Error(e.to_string()))?;

                Ok(Some(file))
            }

            FileAction::Move { src, dst, copy } => {
                let mut path_src = path_file.clone();
                let mut path_dst = path_file.clone();

                path_src.push(&src);
                path_dst.push(&dst);
                {
                    let src = path_src.clone();
                    let dst = path_dst.clone();

                    let mut bytes = Vec::new();

                    let mut file =
                        File::open(src).map_err(|e| FileServiceError::Error(e.to_string()))?;
                    let _ = file.read_to_end(&mut bytes);

                    let mut new_file = File::create_new(dst)
                        .map_err(|e| FileServiceError::Error(e.to_string()))?;

                    new_file
                        .write_all(&bytes)
                        .map_err(|e| FileServiceError::Error(e.to_string()))?;
                }

                if !copy {
                    let _ = fs::remove_file(path_src);
                }

                Ok(None)
            }

            FileAction::Delete { src } => {
                path_file.push(&src);
                fs::remove_file(path_file).map_err(|e| FileServiceError::Error(e.to_string()))?;

                Ok(None)
            }
        }
    }
}
