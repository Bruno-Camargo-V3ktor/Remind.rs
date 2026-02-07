use crate::{
    Repository, RepositoryError, RepositoryResult,
    note::{NoteEntity, NoteRepository},
};
use chrono::{DateTime, Utc};
use domain::models::{Note, NoteId, PropertyId, UserId};
use serde::{Deserialize, Serialize};
use std::{str::FromStr, sync::Arc};
use surrealdb::{RecordId, Surreal, Uuid, engine::any::Any};

// DTOS...
#[derive(Serialize, Deserialize, Debug)]
pub struct NoteQueryDTO {
    pub title: String,
    pub content: String,
    pub color: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_id: RecordId,
    pub propertys: Vec<RecordId>,
}

impl From<&NoteEntity> for NoteQueryDTO {
    fn from(value: &NoteEntity) -> Self {
        let property_ids = value.2.clone();
        let user_id = value.1.clone();
        let values = value.0.clone();

        Self {
            title: values.title,
            content: values.content,
            color: values.color,
            created_at: values.created_at,
            updated_at: values.updated_at,

            user_id: RecordId::from_table_key(
                "users",
                Uuid::from_str(&user_id.0.to_string()).unwrap(),
            ),
            propertys: property_ids
                .into_iter()
                .map(|id| {
                    RecordId::from_table_key(
                        "propertys",
                        Uuid::from_str(&id.0.to_string()).unwrap(),
                    )
                })
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NoteResponseDTO {
    pub id: RecordId,
    pub title: String,
    pub content: String,
    pub color: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_id: RecordId,
    pub propertys: Vec<RecordId>,
}

impl From<NoteResponseDTO> for NoteEntity {
    fn from(value: NoteResponseDTO) -> Self {
        let id_str = &value.id.key().to_string().replace("u", "").replace("'", "");
        let user_str = &value
            .user_id
            .key()
            .to_string()
            .replace("u", "")
            .replace("'", "");

        let propertys = value
            .propertys
            .into_iter()
            .map(|p| {
                let id_str = &p.key().to_string().replace("u", "").replace("'", "");
                PropertyId::from_str(id_str).unwrap()
            })
            .collect();

        let user_id = UserId::from_str(user_str).unwrap();
        let note = Note {
            id: NoteId::from_str(id_str).unwrap(),
            title: value.title,
            color: value.color,
            content: value.content,
            created_at: value.created_at,
            updated_at: value.updated_at,
        };

        (note, user_id, propertys)
    }
}

// Repository Impl
pub struct NoteSurrealDbRepository {
    pub db: Arc<Surreal<Any>>,
}

impl NoteSurrealDbRepository {
    pub fn new(db: Arc<Surreal<Any>>) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl Repository for NoteSurrealDbRepository {
    type Entity = NoteEntity;
    type Id = NoteId;

    async fn get_by_id(&self, id: Self::Id) -> RepositoryResult<Self::Entity> {
        let uuid = Uuid::from_str(&id.0.to_string()).unwrap();
        let op: Option<NoteResponseDTO> = self.db.select(("notes", uuid)).await.unwrap();

        op.map(NoteEntity::from)
            .ok_or(RepositoryError::EntityNotFound(format!(
                "Note with id: u'{uuid}' not found"
            )))
    }

    async fn list(&self, quantity: usize, page: usize) -> RepositoryResult<Vec<Self::Entity>> {
        if page == 0 {
            return Ok(vec![]);
        }

        let index = (page * quantity) - quantity;

        let mut query = self
            .db
            .query("SELECT * FROM notes LIMIT $qtn START $page")
            .bind(("qtn", quantity))
            .bind(("page", index))
            .await
            .map_err(|_| RepositoryError::DatabaseConnection)?;

        match query.take::<Vec<NoteResponseDTO>>(0) {
            Ok(list) => Ok(list.into_iter().map(NoteEntity::from).collect()),
            Err(_) => Err(RepositoryError::Unknow),
        }
    }

    async fn create(&self, entity: Self::Entity) -> RepositoryResult<Self::Entity> {
        let uuid = Uuid::from_str(&entity.0.id.0.to_string()).unwrap();
        let query = NoteQueryDTO::from(&entity);

        let op: Option<NoteResponseDTO> = self
            .db
            .create(("notes", uuid))
            .content(query)
            .await
            .map_err(|e| {
                println!("{e:?}");
                RepositoryError::DatabaseConnection
            })?;

        op.map(NoteEntity::from).ok_or(RepositoryError::Unknow)
    }

    async fn update(&self, new_entity: Self::Entity) -> RepositoryResult<Self::Entity> {
        let uuid = Uuid::from_str(&new_entity.0.id.0.to_string()).unwrap();
        let query = NoteQueryDTO::from(&new_entity);

        let op: Option<NoteResponseDTO> = self
            .db
            .update(("notes", uuid))
            .merge(query)
            .await
            .map_err(|_| RepositoryError::DatabaseConnection)?;

        op.map(NoteEntity::from)
            .ok_or(RepositoryError::EntityNotFound(format!(
                "Note with id: u'{uuid}' not found"
            )))
    }

    async fn delete(&self, id: Self::Id) -> RepositoryResult<()> {
        let uuid = Uuid::from_str(&id.0.to_string()).unwrap();
        let op: Option<NoteResponseDTO> = self
            .db
            .delete(("notes", uuid))
            .await
            .map_err(|_| RepositoryError::DatabaseConnection)?;

        op.map(|_| {})
            .ok_or(RepositoryError::EntityNotFound(format!(
                "Note with id: u'{uuid}' not found"
            )))
    }
}

#[async_trait::async_trait]
impl NoteRepository for NoteSurrealDbRepository {
    async fn list_all_by_user(&self, user_id: UserId) -> RepositoryResult<Vec<Self::Entity>> {
        let mut result = self
            .db
            .query("SELECT * FROM notes WHERE user_id = $user")
            .bind((
                "user",
                RecordId::from_table_key("users", Uuid::from_str(&user_id.0.to_string()).unwrap()),
            ))
            .await
            .map_err(|_| RepositoryError::DatabaseConnection)?;

        match result.take::<Vec<NoteResponseDTO>>(0) {
            Ok(list) => Ok(list.into_iter().map(NoteEntity::from).collect()),
            Err(_) => Err(RepositoryError::Unknow),
        }
    }

    async fn get_by_title(&self, user_id: UserId, title: String) -> RepositoryResult<Self::Entity> {
        let mut result = self
            .db
            .query("SELECT * FROM notes WHERE user_id = $user and title = $title")
            .bind((
                "user",
                RecordId::from_table_key("users", Uuid::from_str(&user_id.0.to_string()).unwrap()),
            ))
            .bind(("title", title.clone()))
            .await
            .map_err(|_| RepositoryError::DatabaseConnection)?;

        match result.take::<Option<NoteResponseDTO>>(0) {
            Ok(res) => match res {
                Some(p) => return Ok(NoteEntity::from(p)),
                None => Err(RepositoryError::EntityNotFound(format!(
                    "Note with title: '{title}' not found"
                ))),
            },
            Err(_) => Err(RepositoryError::Unknow),
        }
    }
}
