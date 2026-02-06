use crate::{app::App, db::connection_db};
use config::load_config;
use repository::{
    note::NoteSurrealDbRepository, property::PropertySurrealDbRepository,
    user::UserSurrealDbRepository,
};
use security::argon2::Argon2Hash;
use services::{
    CreateNoteBuilder, CreatePropertyBuilder, CreateUserBuilder, DeleteNoteBuilder,
    DeletePropertyBuilder, DeleteUserBuilder, ListNoteBuilder, ListPropertyBuilder,
    LocalStorageBuilder, LoginUserBuilder, S3StorageBuilder, SendEmailBuilder, ServiceBuilder,
    UpdateNoteBuilder, UpdatePropertyBuilder, UpdateUserBuilder,
};
use std::{collections::HashMap, sync::Arc};

mod app;
mod db;
mod guards;
mod routers;
mod utils;

#[actix_web::main]
async fn main() {
    let _ = rustls::crypto::ring::default_provider().install_default();

    let _ = dotenv::dotenv();
    let db = connection_db().await;

    let app = App::new(|mut app| async {
        let config = load_config();

        if let Some(config) = &config.s3_storage {
            app.add_service(
                S3StorageBuilder::new()
                    .url(&config.url)
                    .access_key_id(&config.access_key_id)
                    .access_key_secret(&config.access_key_secret)
                    .provide(&config.provide)
                    .region(&config.region)
                    .temp_files_path(&config.temp_files_path)
                    .build(),
            )
            .await;
        }

        let password_hash = Arc::new(Argon2Hash::default());

        let user_repo = Arc::new(UserSurrealDbRepository::new(db.clone()));
        let property_repo = Arc::new(PropertySurrealDbRepository::new(db.clone()));
        let note_repo = Arc::new(NoteSurrealDbRepository::new(db.clone()));

        app.user_repo(user_repo.clone());
        app.property_repo(property_repo.clone());
        app.note_repo(note_repo.clone());

        app.password_hash(password_hash.clone());

        app.add_table_errors_code(HashMap::from([
            ("EMAIL_ALREADY_EXISTS".into(), 409),
            ("NOTE_ALREADY_EXISTS".into(), 409),
            ("PROPERTY_ALREADY_EXISTS".into(), 409),
            ("INVALID_FIELDS".into(), 400),
            ("DATABASE_ERROR".into(), 500),
            ("SEND_EMAIL_FAIL".into(), 500),
            ("INTERNAL_SERVER_ERROR".into(), 500),
            ("INVALID_CREDENTIALS".into(), 401),
            ("USER_NOT_EXIST".into(), 404),
            ("PROPERTY_NOT_EXIST".into(), 404),
            ("NOTE_NOT_EXIST".into(), 404),
            ("INVALID_TOKEN".into(), 401),
            ("IO_SERVER_ERROR".into(), 500),
        ]));

        app.add_service(
            LocalStorageBuilder::new()
                .base(config.local_storage.storage_dir.clone())
                .build(),
        )
        .await;

        app.add_service(
            SendEmailBuilder::new()
                .from(
                    config.email.from_name.clone(),
                    config.email.from_email.clone(),
                )
                .cred(config.email.app_key.clone())
                .smtp(config.email.smtp.clone())
                .build(),
        )
        .await;

        app.add_service(
            CreateUserBuilder::new()
                .password_hash(password_hash.clone())
                .user_repository(user_repo.clone())
                .build(),
        )
        .await;

        app.add_service(
            LoginUserBuilder::new()
                .password_hash(password_hash.clone())
                .user_repository(user_repo.clone())
                .build(),
        )
        .await;

        app.add_service(
            DeleteUserBuilder::new()
                .user_repository(user_repo.clone())
                .build(),
        )
        .await;

        app.add_service(
            UpdateUserBuilder::new()
                .user_repository(user_repo.clone())
                .password_hash(password_hash.clone())
                .build(),
        )
        .await;

        app.add_service(
            CreatePropertyBuilder::new()
                .repo_property(property_repo.clone())
                .build(),
        )
        .await;

        app.add_service(
            UpdatePropertyBuilder::new()
                .repo_property(property_repo.clone())
                .build(),
        )
        .await;

        app.add_service(
            DeletePropertyBuilder::new()
                .repo_property(property_repo.clone())
                .build(),
        )
        .await;

        app.add_service(
            ListPropertyBuilder::new()
                .repo_property(property_repo.clone())
                .build(),
        )
        .await;

        app.add_service(
            CreateNoteBuilder::new()
                .note_repo(note_repo.clone())
                .build(),
        )
        .await;

        app.add_service(
            UpdateNoteBuilder::new()
                .note_repo(note_repo.clone())
                .build(),
        )
        .await;

        app.add_service(
            DeleteNoteBuilder::new()
                .note_repo(note_repo.clone())
                .build(),
        )
        .await;

        app.add_service(ListNoteBuilder::new().note_repo(note_repo.clone()).build())
            .await;

        app.config(config);
        app.build()
    })
    .await;

    let _ = app.run().await;
}
