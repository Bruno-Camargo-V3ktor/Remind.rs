use crate::config::load_config;
use std::sync::Arc;
use surrealdb::{Surreal, engine::any::Any, opt::auth::Root};
use surrealdb_migrations::MigrationRunner;

pub async fn connection_db() -> Arc<Surreal<Any>> {
    let surreal_config = load_config()
        .surreal_db
        .expect("Missing configurations for SurrealDB Connection");

    let db = Surreal::<Any>::init();
    db.connect(surreal_config.url)
        .await
        .expect("Falied by connect for Surreal DB");

    db.signin(Root {
        username: &surreal_config.username,
        password: &surreal_config.password,
    })
    .await
    .expect("Falied singin in SurrealDB root");

    db.use_ns(surreal_config.namespace)
        .use_db(surreal_config.database)
        .await
        .expect("Invalid namespace or database name");

    MigrationRunner::new(&db)
        .up()
        .await
        .expect("Failed to apply migrations");

    Arc::new(db)
}
