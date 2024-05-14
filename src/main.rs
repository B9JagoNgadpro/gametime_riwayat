pub mod model;
pub mod repository;
pub mod service;
pub mod controller;

use actix_web::{web, App, HttpServer, HttpResponse};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use dotenv::dotenv;
use std::env;

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

async fn get_db_pool() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let port = env::var("PORT").expect("Missing port number");
    let port = port.parse::<u16>().expect("Invalid port given");

    let pool = get_db_pool().await;

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .route("/", web::get().to(index))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
