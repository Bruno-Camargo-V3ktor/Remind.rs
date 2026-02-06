use super::super::{FileAction, FileServiceError};
use crate::{Service, TempFile, file::content_type};

use aws_config::{self, Region};
use aws_sdk_s3::{self as s3, primitives::ByteStream};
use chrono::Utc;

pub struct S3StorageService {
    pub url: String,
    pub access_key_id: String,
    pub access_key_secret: String,
    pub provide: String,
    pub region: String,
    pub temp_files_path: String,
}

#[async_trait::async_trait]
impl Service for S3StorageService {
    type Args = FileAction;
    type Out = Option<TempFile>;

    async fn run(&self, args: Self::Args) -> Result<Self::Out, FileServiceError> {
        let config = aws_config::from_env()
            .endpoint_url(self.url.clone())
            .credentials_provider(s3::config::Credentials::new(
                self.access_key_id.clone(),
                self.access_key_secret.clone(),
                None,
                None,
                Box::leak(self.provide.clone().into_boxed_str()),
            ))
            .region(Region::new(self.region.clone()))
            .load()
            .await;

        let client = s3::Client::new(&config);

        match args {
            FileAction::Save { bytes, dst } => {
                let (bucket, key) = dst.split_once("/").unwrap_or(("", ""));
                let content_type = content_type(key);

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
                let (bucket, key) = path.split_once("/").unwrap_or(("", ""));

                let res = client.get_object().bucket(bucket).key(key).send().await;

                if let Err(e) = res {
                    return Err(FileServiceError::Error(e.to_string()));
                }

                let data = res.unwrap().body.collect().await.unwrap();
                let bytes = data.into_bytes().into_iter().collect::<Vec<u8>>();

                let file_path = format!(
                    "{}/{}_{}_{}",
                    self.temp_files_path,
                    bucket,
                    Utc::now().timestamp(),
                    key.split("/").last().unwrap_or("")
                );
                let temp_file = TempFile::from_bytes(bytes, file_path);

                match temp_file {
                    Ok(file) => Ok(Some(file)),
                    Err(e) => Err(FileServiceError::Error(e.to_string())),
                }
            }

            FileAction::Move { src, dst, copy } => {
                let (bucket_dst, key_dst) = dst.split_once("/").unwrap_or(("", ""));

                let res = client
                    .copy_object()
                    .copy_source(&src)
                    .bucket(bucket_dst)
                    .key(key_dst)
                    .send()
                    .await;

                if let Err(e) = res {
                    return Err(FileServiceError::Error(e.to_string()));
                }

                if !copy {
                    let (bucket, key) = src.split_once("/").unwrap_or(("", ""));

                    let res = client.delete_object().bucket(bucket).key(key).send().await;

                    if let Err(e) = res {
                        return Err(FileServiceError::Error(e.to_string()));
                    }
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
