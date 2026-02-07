use crate::{
    Repository, RepositoryError, RepositoryResult,
    property::{PropertyEntity, PropertyRepository},
};
use chrono::{DateTime, Utc};
use domain::models::{Property, PropertyId, PropertyTypes, UserId};
use serde::{Deserialize, Serialize};
use std::{str::FromStr, sync::Arc};
use surrealdb::{RecordId, Surreal, Uuid, engine::any::Any};

// DTOS...
#[derive(Serialize, Deserialize, Debug)]
pub struct PropertyQueryDTO {
    #[serde(rename = "type")]
    pub r#type: PropertyTypes,
    pub name: String,
    pub color: u32,
    pub value: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_id: RecordId,
}

impl From<&PropertyEntity> for PropertyQueryDTO {
    fn from(value: &PropertyEntity) -> Self {
        let user_id = value.1.clone();
        let values = value.0.clone();

        Self {
            r#type: values.r#type,
            name: values.name,
            color: values.color,
            value: values.value,
            created_at: values.created_at,
            updated_at: values.updated_at,
            user_id: RecordId::from_table_key(
                "users",
                Uuid::from_str(&user_id.0.to_string()).unwrap(),
            ),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PropertyResponseDTO {
    pub id: RecordId,
    #[serde(rename = "type")]
    pub r#type: PropertyTypes,
    pub name: String,
    pub color: u32,
    pub value: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_id: RecordId,
}

impl From<PropertyResponseDTO> for PropertyEntity {
    fn from(value: PropertyResponseDTO) -> Self {
        let id_str = &value.id.key().to_string().replace("u", "").replace("'", "");
        let user_str = &value
            .user_id
            .key()
            .to_string()
            .replace("u", "")
            .replace("'", "");

        let user_id = UserId::from_str(user_str).unwrap();
        let property = Property {
            id: PropertyId::from_str(id_str).unwrap(),
            r#type: value.r#type,
            name: value.name,
            color: value.color,
            value: value.value,
            created_at: value.created_at,
            updated_at: value.updated_at,
        };

        (property, user_id)
    }
}

// Repository Impl
pub struct PropertySurrealDbRepository {
    pub db: Arc<Surreal<Any>>,
}

impl PropertySurrealDbRepository {
    pub fn new(db: Arc<Surreal<Any>>) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl Repository for PropertySurrealDbRepository {
    type Entity = PropertyEntity;
    type Id = PropertyId;

    async fn get_by_id(&self, id: Self::Id) -> RepositoryResult<Self::Entity> {
        let uuid = Uuid::from_str(&id.0.to_string()).unwrap();
        let op: Option<PropertyResponseDTO> = self.db.select(("propertys", uuid)).await.unwrap();

        op.map(PropertyEntity::from)
            .ok_or(RepositoryError::EntityNotFound(format!(
                "Property with id: u'{uuid}' not found"
            )))
    }

    async fn list(&self, quantity: usize, page: usize) -> RepositoryResult<Vec<Self::Entity>> {
        if page == 0 {
            return Ok(vec![]);
        }

        let index = (page * quantity) - quantity;

        let mut query = self
            .db
            .query("SELECT * FROM propertys LIMIT $qtn START $page")
            .bind(("qtn", quantity))
            .bind(("page", index))
            .await
            .map_err(|_| RepositoryError::DatabaseConnection)?;

        match query.take::<Vec<PropertyResponseDTO>>(0) {
            Ok(list) => Ok(list.into_iter().map(PropertyEntity::from).collect()),
            Err(_) => Err(RepositoryError::Unknow),
        }
    }

    async fn create(&self, entity: Self::Entity) -> RepositoryResult<Self::Entity> {
        let uuid = Uuid::from_str(&entity.0.id.0.to_string()).unwrap();
        let query: PropertyQueryDTO = PropertyQueryDTO::from(&entity);

        let op: Option<PropertyResponseDTO> = self
            .db
            .create(("propertys", uuid))
            .content(query)
            .await
            .map_err(|e| {
                println!("{e:?}");
                RepositoryError::DatabaseConnection
            })?;

        op.map(PropertyEntity::from).ok_or(RepositoryError::Unknow)
    }

    async fn update(&self, mut new_entity: Self::Entity) -> RepositoryResult<Self::Entity> {
        let uuid = Uuid::from_str(&new_entity.0.id.0.to_string()).unwrap();

        new_entity.0.updated_at = Utc::now();
        let query: PropertyQueryDTO = PropertyQueryDTO::from(&new_entity);

        let op: Option<PropertyResponseDTO> = self
            .db
            .update(("propertys", uuid))
            .merge(query)
            .await
            .map_err(|_| RepositoryError::DatabaseConnection)?;

        op.map(PropertyEntity::from)
            .ok_or(RepositoryError::EntityNotFound(format!(
                "Property with id: u'{uuid}' not found"
            )))
    }

    async fn delete(&self, id: Self::Id) -> RepositoryResult<()> {
        let uuid = Uuid::from_str(&id.0.to_string()).unwrap();
        let op: Option<PropertyResponseDTO> = self
            .db
            .delete(("propertys", uuid))
            .await
            .map_err(|_| RepositoryError::DatabaseConnection)?;

        op.map(|_| {})
            .ok_or(RepositoryError::EntityNotFound(format!(
                "Property with id: u'{uuid}' not found"
            )))
    }
}

#[async_trait::async_trait]
impl PropertyRepository for PropertySurrealDbRepository {
    async fn list_all_by_user(&self, user_id: UserId) -> RepositoryResult<Vec<Self::Entity>> {
        let mut result = self
            .db
            .query("SELECT * FROM propertys WHERE user_id = $user")
            .bind((
                "user",
                RecordId::from_table_key("users", Uuid::from_str(&user_id.0.to_string()).unwrap()),
            ))
            .await
            .map_err(|_| RepositoryError::DatabaseConnection)?;

        match result.take::<Vec<PropertyResponseDTO>>(0) {
            Ok(list) => Ok(list.into_iter().map(PropertyEntity::from).collect()),
            Err(_) => Err(RepositoryError::Unknow),
        }
    }

    async fn get_by_name(&self, user_id: UserId, name: String) -> RepositoryResult<Self::Entity> {
        let mut result = self
            .db
            .query("SELECT * FROM propertys WHERE user_id = $user and name = $name")
            .bind((
                "user",
                RecordId::from_table_key("users", Uuid::from_str(&user_id.0.to_string()).unwrap()),
            ))
            .bind(("name", name.clone()))
            .await
            .map_err(|_| RepositoryError::DatabaseConnection)?;

        match result.take::<Option<PropertyResponseDTO>>(0) {
            Ok(res) => match res {
                Some(p) => return Ok(PropertyEntity::from(p)),
                None => Err(RepositoryError::EntityNotFound(format!(
                    "Property with name: {name}' not found"
                ))),
            },
            Err(_) => Err(RepositoryError::Unknow),
        }
    }
}
