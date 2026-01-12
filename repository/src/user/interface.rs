use domain::models::{User, UserId};

use crate::Repository;

#[async_trait::async_trait]
pub trait UserRepository: Repository<Entity = User, Id = UserId> {}
