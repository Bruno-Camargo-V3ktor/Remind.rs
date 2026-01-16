use crate::{
    Repository, RepositoryError, RepositoryResult,
    user::{UserEntity, UserRepository},
};
use domain::models::{User, UserId};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

pub struct UserInMemoryRepository {
    pub registres: Arc<RwLock<HashMap<UserId, User>>>,
}

impl UserInMemoryRepository {
    pub fn new(values: impl IntoIterator<Item = (UserId, User)>) -> Self {
        Self {
            registres: Arc::new(RwLock::new(HashMap::from_iter(values))),
        }
    }
}

#[async_trait::async_trait]
impl Repository for UserInMemoryRepository {
    type Entity = User;
    type Id = UserId;

    async fn get_by_id(&self, id: Self::Id) -> RepositoryResult<Self::Entity> {
        let registers = self.registres.read().await;

        registers
            .get(&id)
            .ok_or_else(|| RepositoryError::EntityNotFound(format!("User - {id:?}")))
            .cloned()
    }
    async fn list(&self, quantity: usize, page: usize) -> RepositoryResult<Vec<Self::Entity>> {
        let registers = self.registres.read().await;
        let list: Vec<_> = registers.values().cloned().collect();

        let start = page * quantity;
        let end = (start + quantity).clamp(start, list.len() - 1);

        Ok(list[start..=end].to_vec())
    }
    async fn create(&self, entity: Self::Entity) -> RepositoryResult<Self::Entity> {
        let mut registers = self.registres.write().await;

        registers.insert(entity.id.clone(), entity.clone());
        Ok(entity)
    }
    async fn update(&self, new_entity: Self::Entity) -> RepositoryResult<Self::Entity> {
        let mut registers = self.registres.write().await;

        registers.insert(new_entity.id.clone(), new_entity.clone());
        Ok(new_entity)
    }
    async fn delete(&self, id: Self::Id) -> RepositoryResult<()> {
        let mut registers = self.registres.write().await;

        registers
            .remove(&id)
            .map(|_| ())
            .ok_or_else(|| RepositoryError::EntityNotFound(format!("User - {id:?}")))
    }
}

#[async_trait::async_trait]
impl UserRepository for UserInMemoryRepository {
    async fn get_by_email(&self, _email: String) -> RepositoryResult<UserEntity> {
        todo!()
    }
}
