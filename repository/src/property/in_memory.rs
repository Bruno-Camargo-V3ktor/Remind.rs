use crate::{Repository, RepositoryError, RepositoryResult, property::PropertyRepository};
use domain::models::{Property, PropertyId, UserId};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

pub struct PropertyInMemoryRepository {
    pub registres: Arc<RwLock<HashMap<PropertyId, (Property, UserId)>>>,
}

impl PropertyInMemoryRepository {
    pub fn new(values: impl IntoIterator<Item = (PropertyId, (Property, UserId))>) -> Self {
        Self {
            registres: Arc::new(RwLock::new(HashMap::from_iter(values))),
        }
    }
}

#[async_trait::async_trait]
impl Repository for PropertyInMemoryRepository {
    type Entity = (Property, UserId);
    type Id = PropertyId;

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

        registers.insert(entity.0.id.clone(), entity.clone());
        Ok(entity)
    }
    async fn update(&self, new_entity: Self::Entity) -> RepositoryResult<Self::Entity> {
        let mut registers = self.registres.write().await;

        registers.insert(new_entity.0.id.clone(), new_entity.clone());
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
impl PropertyRepository for PropertyInMemoryRepository {
    async fn list_all_by_user(&self, user_id: UserId) -> RepositoryResult<Vec<Self::Entity>> {
        let registers = self.registres.read().await;
        let list: Vec<_> = registers
            .values()
            .filter(|e| e.1 == user_id)
            .cloned()
            .collect();

        Ok(list)
    }

    async fn get_by_name(&self, _user_id: UserId, _name: String) -> RepositoryResult<Self::Entity> {
        todo!()
    }
}
