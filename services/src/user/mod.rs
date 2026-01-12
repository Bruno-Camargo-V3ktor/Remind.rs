mod create_user;

use repository::user::UserRepository;
use std::sync::Arc;

pub use create_user::*;

pub type UserRepositoryType = Arc<dyn UserRepository + 'static + Send + Sync>;
