use actix_web::{App, HttpServer, Responder, get};
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("0.0.0.0:3000")?
        .run()
        .await?;

    Ok(())
}

#[get("/")]
async fn index() -> impl Responder {
    "Hello"
}
