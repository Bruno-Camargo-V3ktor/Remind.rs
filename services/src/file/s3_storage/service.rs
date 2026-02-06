use super::super::{FileAction, FileServiceError};
use crate::{Service, file::content_type};
use std::{
    fs::{self, File},
    io::{Read, Write},
};

use aws_config;
use aws_sdk_s3::{self as s3, primitives::ByteStream};

pub struct S3StorageService {
    pub url: String,
    pub access_key_id: String,
    pub access_key_secret: String,
    pub provide: String,
    pub region: String,
}

#[async_trait::async_trait]
impl Service for S3StorageService {
    type Args = FileAction;
    type Out = Option<File>;

    async fn run(&self, args: Self::Args) -> Result<Self::Out, FileServiceError> {
        let config = aws_config::from_env()
            .endpoint_url(self.url.clone())
            .credentials_provider(s3::config::Credentials::new(
                self.access_key_id.clone(),
                self.access_key_secret.clone(),
                None,
                None,
                &self.provide,
            ))
            .region(self.region.as_str())
            .load()
            .await;

        let client = s3::Client::new(&config);

        match args {
            FileAction::Save { bytes, dst } => {
                let (bucket, key) = dst.split_once("/").unwrap_or(("", ""));
                let content_type = content_type(&key);

                let body = ByteStream::from(bytes);

                let res = client
                    .put_object()
                    .bucket(bucket)
                    .key(key)
                    .body(body)
                    .content_type(content_type)
                    .send()
                    .await;

                if let Err(e) = res {
                    return Err(FileServiceError::Error(e.to_string()));
                }

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
                let (bucket, key) = src.split_once("/").unwrap_or(("", ""));

                let res = client.delete_object().bucket(bucket).key(key).send().await;

                if let Err(e) = res {
                    return Err(FileServiceError::Error(e.to_string()));
                }

                Ok(None)
            }
        }
    }
}
