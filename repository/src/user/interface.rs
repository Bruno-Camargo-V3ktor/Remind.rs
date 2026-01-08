use crate::Repository;

#[async_trait::async_trait]
pub trait UserRepository: Repository {}
