pub mod adapter;
pub mod controller;
pub mod model;
pub mod repository;
pub mod service;

use std::env;
use dotenv::dotenv;
use actix_web::{ web, App, HttpServer };
use actix_cors::Cors;
use repository::transaksi_repository::TransaksiRepository;
use service::transaksi_service::TransaksiService;
use sqlx::postgres::{PgPool, PgPoolOptions };
use crate::controller::transaksi_controller::{create_transaksi, get_user_transactions, get_transaction_game_info_by_penjual};
use crate::adapter::redis_adapter::RedisAdapter;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port = env::var("PORT").expect("Missing port number");
    let port = port.parse::<u16>().expect("Invalid port given");

    let database_url = env::var("DATABASE_URL").expect("Missing database URL");
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");

    let pool: PgPool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .unwrap();
    let redis_adapter = RedisAdapter::new(&redis_url).await;

    let repository = TransaksiRepository { pool };
    let service = web::Data::new(TransaksiService::new(repository, redis_adapter));

    HttpServer::new(move || {
        App::new()
            .app_data(service.clone())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600)
            )
            .route("/", web::get().to(|| async { "Hello, World!" }))
            .route("/create", web::post().to(create_transaksi))
            .route("/get/{user_id}", web::get().to(get_user_transactions))
            .route("/get-penjual/{penjual_id}", web::get().to(get_transaction_game_info_by_penjual))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
