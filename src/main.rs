pub mod model;
pub mod repository;
pub mod service;
pub mod controller;

use actix_web::{web, App, HttpServer, HttpResponse};

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
