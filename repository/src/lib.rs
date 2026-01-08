use thiserror::Error;

pub mod user;

pub type RepositoryResult<T> = Result<RepositoryError, T>;

#[async_trait::async_trait]
pub trait Repository {
    type Entity;
    type Id;

    async fn get_by_id(&self, id: Self::Id) -> RepositoryResult<Self::Entity>;
    async fn list(&self, quantity: usize, page: usize) -> RepositoryResult<Vec<Self::Entity>>;
    async fn create(&self, user: Self::Entity) -> RepositoryResult<Self::Entity>;
    async fn update(&self, new_user: Self::Entity) -> RepositoryResult<Self::Entity>;
    async fn delete(&self, id: Self::Id) -> RepositoryResult<()>;
}

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("The entity '{0}' not found")]
    EntityNotFound(String),
    #[error("Database not responde")]
    DatabaseConnection,
    #[error("Invalid query send to database")]
    DataError,
    #[error("Unknown error repository")]
    Unknow,
}
