use repository::user::UserRepository;
use std::sync::Arc;

mod create_user;
mod login_user;

pub use create_user::*;
pub use login_user::*;

pub type UserRepositoryType = Arc<dyn UserRepository + 'static + Send + Sync>;
