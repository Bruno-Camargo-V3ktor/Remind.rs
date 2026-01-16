use crate::{
    Repository, RepositoryError, RepositoryResult,
    user::{UserEntity, UserRepository},
};
use chrono::{DateTime, Utc};
use domain::models::{User, UserId};
use serde::{Deserialize, Serialize};
use std::{str::FromStr, sync::Arc};
use surrealdb::{RecordId, Surreal, Uuid, engine::any::Any};

#[derive(Serialize, Deserialize, Debug)]
struct UserQueryDTO {
    pub name: String,
    pub email: String,
    pub password: String,
    pub bio: String,
    pub photo_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<&UserEntity> for UserQueryDTO {
    fn from(value: &UserEntity) -> Self {
        Self {
            name: value.name.clone(),
            email: value.email.clone(),
            bio: value.bio.clone(),
            password: value.password.clone(),
            photo_url: value.photo_url.clone(),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct UserResponseDTO {
    pub id: RecordId,
    pub name: String,
    pub email: String,
    pub password: String,
    pub bio: String,
    pub photo_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<UserResponseDTO> for UserEntity {
    fn from(value: UserResponseDTO) -> Self {
        let uuid_str = &value.id.key().to_string().replace("u", "").replace("'", "");

        Self {
            id: UserId::from_str(uuid_str).unwrap(),
            name: value.name.clone(),
            email: value.email.clone(),
            bio: value.bio.clone(),
            password: value.password.clone(),
            photo_url: value.photo_url.clone(),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

pub struct UserSurrealDbRepository {
    pub db: Arc<Surreal<Any>>,
}

impl UserSurrealDbRepository {
    pub fn new(db: Arc<Surreal<Any>>) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl Repository for UserSurrealDbRepository {
    type Entity = User;
    type Id = UserId;

    async fn get_by_id(&self, id: Self::Id) -> RepositoryResult<Self::Entity> {
        let uuid = Uuid::from_str(&id.0.to_string()).unwrap();
        let op_user: Option<UserResponseDTO> = self.db.select(("users", uuid)).await.unwrap();

        op_user
            .map(UserEntity::from)
            .ok_or(RepositoryError::EntityNotFound(format!("User - {uuid}")))
    }

    async fn list(&self, quantity: usize, page: usize) -> RepositoryResult<Vec<Self::Entity>> {
        if page == 0 {
            return Ok(vec![]);
        }

        let index = (page * quantity) - quantity;

        let mut query = self
            .db
            .query("SELECT * FROM users LIMIT $qtn START $page")
            .bind(("qtn", quantity))
            .bind(("page", index))
            .await
            .map_err(|_| RepositoryError::DatabaseConnection)?;

        match query.take::<Vec<UserResponseDTO>>(0) {
            Ok(list) => Ok(list.into_iter().map(UserEntity::from).collect()),
            Err(_) => Err(RepositoryError::Unknow),
        }
    }

    async fn create(&self, entity: Self::Entity) -> RepositoryResult<Self::Entity> {
        let user_query: UserQueryDTO = UserQueryDTO::from(&entity);

        let op_user: Option<UserResponseDTO> = self
            .db
            .create(("users", entity.id.0))
            .content(user_query)
            .await
            .map_err(|e| {
                println!("{e:?}");
                RepositoryError::DatabaseConnection
            })?;

        op_user
            .map(UserEntity::from)
            .ok_or(RepositoryError::EntityNotFound(format!(
                "User - {}",
                entity.id.0
            )))
    }

    async fn update(&self, new_entity: Self::Entity) -> RepositoryResult<Self::Entity> {
        let uuid = Uuid::from_str(&new_entity.id.0.to_string()).unwrap();
        let user_query: UserQueryDTO = UserQueryDTO::from(&new_entity);

        let op_user: Option<UserResponseDTO> = self
            .db
            .update(("users", uuid))
            .merge(user_query)
            .await
            .map_err(|_| RepositoryError::DatabaseConnection)?;

        op_user
            .map(UserEntity::from)
            .ok_or(RepositoryError::EntityNotFound(format!("User - {uuid}")))
    }

    async fn delete(&self, id: Self::Id) -> RepositoryResult<()> {
        let uuid = Uuid::from_str(&id.0.to_string()).unwrap();
        let op_user: Option<UserResponseDTO> = self
            .db
            .delete(("users", uuid))
            .await
            .map_err(|_| RepositoryError::DatabaseConnection)?;

        op_user
            .map(|_| {})
            .ok_or(RepositoryError::EntityNotFound(format!("User - {uuid}")))
    }
}

#[async_trait::async_trait]
impl UserRepository for UserSurrealDbRepository {
    async fn get_by_email(&self, email: String) -> RepositoryResult<UserEntity> {
        let email = email.clone();

        let mut query = self
            .db
            .query("SELECT * FROM users WHERE email = $email LIMIT 1")
            .bind(("email", email.clone()))
            .await
            .map_err(|_| RepositoryError::DatabaseConnection)?;

        match query.take::<Option<UserResponseDTO>>(0) {
            Ok(user) => {
                if let Some(u) = user {
                    Ok(UserEntity::from(u))
                } else {
                    Err(RepositoryError::EntityNotFound(format!(
                        "User with email: {email} not found"
                    )))
                }
            }
            Err(_) => Err(RepositoryError::Unknow),
        }
    }
}
