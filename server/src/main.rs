use crate::{app::App, config::load_config, db::connection_db};
use repository::{
    note::NoteSurrealDbRepository, property::PropertySurrealDbRepository,
    user::UserSurrealDbRepository,
};
use security::argon2::Argon2Hash;
use services::{
    CreateUserBuilder, DeleteUserBuilder, LoginUserBuilder, SendEmailBuilder, ServiceBuilder,
};
use std::{collections::HashMap, sync::Arc};

mod app;
mod config;
mod db;
mod guards;
mod routers;

#[actix_web::main]
async fn main() {
    let _ = dotenv::dotenv();
    let db = connection_db().await;

    let app = App::new(|mut app| async {
        let config = load_config();

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
            ("INVALID_FIELDS".into(), 400),
            ("DATABASE_ERROR".into(), 500),
            ("SEND_EMAIL_FAIL".into(), 500),
            ("INTERNAL_SERVER_ERROR".into(), 500),
            ("INVALID_CREDENTIALS".into(), 401),
            ("USER_NOT_EXIST".into(), 404),
            ("INVALID_TOKEN".into(), 401),
        ]));

        app.add_service(
            SendEmailBuilder::new()
                .from(
                    config.email.from_name.clone(),
                    config.email.from_email.clone(),
                )
                .cred(config.email.username.clone(), config.email.password.clone())
                .smtp(config.email.smtp)
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

        app.config(config);
        app.build()
    })
    .await;

    let _ = app.run().await;
}
