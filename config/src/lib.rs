mod app;
mod email;
mod local_storage;
mod s3_storage;
mod security;
mod server;
mod surreal_db;

pub use app::*;
pub use email::*;
pub use local_storage::*;
pub use s3_storage::*;
pub use security::*;
pub use server::*;
pub use surreal_db::*;

use figment::{
    Figment,
    providers::{Env, Format, Json},
};

pub fn load_config() -> ConfigApp {
    Figment::new()
        .merge(Env::prefixed("REMIND_").split("__"))
        .merge(Json::file("config.json"))
        .extract::<ConfigApp>()
        .expect("Invalid Configuration")
}
