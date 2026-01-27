use crate::{app::App, config::load_config, db::connection_db};
use repository::{
    note::NoteSurrealDbRepository, property::PropertySurrealDbRepository,
    user::UserSurrealDbRepository,
};
use services::CreateUserService;
use std::{collections::HashMap, sync::Arc};

mod app;
mod config;
mod db;
mod routers;

#[actix_web::main]
async fn main() {
    let _ = dotenv::dotenv();
    let db = connection_db().await;

    let app = App::new(|mut app| async {
        let config = load_config();
        app.config(config);

        let user_repo = Arc::new(UserSurrealDbRepository::new(db.clone()));
        let property_repo = Arc::new(PropertySurrealDbRepository::new(db.clone()));
        let note_repo = Arc::new(NoteSurrealDbRepository::new(db.clone()));

        app.add_service(CreateUserService::builder(user_repo.clone()))
            .await;

        app.user_repo(user_repo);
        app.property_repo(property_repo);
        app.note_repo(note_repo);

        app.add_table_errors_code(HashMap::from([
            ("EMAIL_ALREADY_EXISTS".into(), 409),
            ("INVALID_FIELDS".into(), 400),
            ("DATABASE_ERROR".into(), 500),
            ("INTERNAL_SERVER_ERROR".into(), 500),
        ]));

        app.build()
    })
    .await;

    let _ = app.run().await;
}
