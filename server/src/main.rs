use crate::{app::App, config::load_config, db::connection_db};
use repository::{
    note::NoteInMemoryRepository,
    property::PropertyInMemoryRepository,
    user::{UserRepository, UserSurrealDbRepository},
};
use services::CreateUserService;
use std::sync::Arc;

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
        let property_repo = Arc::new(PropertyInMemoryRepository::new(vec![]));
        let note_repo = Arc::new(NoteInMemoryRepository::new(vec![]));

        let user01 = user_repo.get_by_email("bruno@camargo.com".into()).await;
        let user02 = user_repo.get_by_email("larissa@camargo.com".into()).await;

        println!("user01: {user01:#?}\n");
        println!("user02: {user02:#?}\n");

        app.add_service(CreateUserService::builder(user_repo.clone()))
            .await;

        app.user_repo(user_repo);
        app.property_repo(property_repo);
        app.note_repo(note_repo);

        app.build()
    })
    .await;

    let _ = app.run().await;
}
