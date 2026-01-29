mod builder;
mod error;
mod service;

pub use builder::*;
pub use error::*;
pub use service::*;

pub struct From {
    name: String,
    email: String,
}

pub struct To {
    name: String,
    email: String,
}

pub struct Cred {
    username: String,
    password: String,
}
