pub mod model;
pub mod repository;
pub mod service;
pub mod controller;

use std::env;
use dotenv::dotenv;
use actix_web::{ web, App, HttpServer };
use repository::transaksi_repository::TransaksiRepository;
use service::transaksi_service::TransaksiService;
use sqlx::postgres::{PgPool, PgPoolOptions };
use crate::controller::transaksi_controller::{create_transaksi, get_user_transactions};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port = env::var("PORT").expect("Missing port number");
    let port = port.parse::<u16>().expect("Invalid port given");

    let database_url = env::var("DATABASE_URL").expect("Missing database URL");

    let pool: PgPool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .unwrap();

    let repository = TransaksiRepository { pool };
    let service = web::Data::new(TransaksiService::new(repository));

    HttpServer::new(move || {
        App::new()
            .app_data(service.clone())
            .route("/", web::get().to(|| async { "Hello, World!" }))
            .route("/create", web::post().to(create_transaksi))
            .route("/get/{user_id}", web::get().to(get_user_transactions))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}