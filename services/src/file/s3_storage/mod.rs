mod builder;
mod service;

use std::{
    fs::{self, File},
    io::{Result as ioResult, Write},
    ops::Deref,
};

pub use builder::*;
pub use service::*;

pub struct TempFile {
    file: Option<File>,
    path: String,
}

impl TempFile {
    pub fn from_bytes(bytes: Vec<u8>, path: impl Into<String>) -> ioResult<Self> {
        let path = path.into();
        let mut new_file = File::create_new(&path)?;

        new_file.write_all(&bytes)?;

        Ok(Self {
            file: Some(new_file),
            path,
        })
    }
}

impl Deref for TempFile {
    type Target = File;

    fn deref(&self) -> &Self::Target {
        self.file.as_ref().unwrap()
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        {
            self.file.take();
        }

        let _ = fs::remove_file(self.path.clone());
    }
}
