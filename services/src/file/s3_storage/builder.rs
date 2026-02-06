use crate::{S3StorageService, ServiceBuilder};

#[derive(Default)]
pub struct S3StorageBuilder {
    url: Option<String>,
    access_key_id: Option<String>,
    access_key_secret: Option<String>,
    provide: Option<String>,
    region: Option<String>,
}

impl S3StorageBuilder {
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn access_key_id(mut self, access_key_id: impl Into<String>) -> Self {
        self.access_key_id = Some(access_key_id.into());
        self
    }

    pub fn access_key_secret(mut self, access_key_secret: impl Into<String>) -> Self {
        self.access_key_secret = Some(access_key_secret.into());
        self
    }

    pub fn provide(mut self, provide: impl Into<String>) -> Self {
        self.provide = Some(provide.into());
        self
    }

    pub fn region(mut self, region: impl Into<String>) -> Self {
        self.region = Some(region.into());
        self
    }
}

impl ServiceBuilder for S3StorageBuilder {
    type S = S3StorageService;

    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn build(self) -> Self::S {
        let url = self.url.expect("");
        let access_key_id = self.access_key_id.expect("");
        let access_key_secret = self.access_key_secret.expect("");
        let provide = self.provide.expect("");
        let region = self.region.expect("");

        S3StorageService {
            url,
            access_key_id,
            access_key_secret,
            provide,
            region,
        }
    }
}
