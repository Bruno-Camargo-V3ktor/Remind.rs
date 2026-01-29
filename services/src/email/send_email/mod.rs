mod builder;
mod error;
mod service;

pub use builder::*;
pub use error::*;
pub use service::*;

pub struct From {
    pub name: String,
    pub email: String,
}

pub struct To {
    pub name: String,
    pub email: String,
}

pub struct Cred {
    pub app_key: String,
}
