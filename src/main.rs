mod controllers;
mod services;

use controllers::hello_controller;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello_controller::hello)
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}