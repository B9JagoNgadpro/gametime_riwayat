pub mod model;
pub mod repository;
pub mod service;
pub mod controller;

use actix_web::{web, App, HttpResponse, HttpServer, Responder, dev::Service};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use dotenv::dotenv;
use std::env;
use prometheus::{Encoder, TextEncoder, IntCounter, IntCounterVec, Registry, opts};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
    pub static ref HTTP_COUNTER: IntCounter = IntCounter::new("http_requests_total", "Total number of HTTP requests made.").unwrap();
    pub static ref HTTP_COUNTER_VEC: IntCounterVec = IntCounterVec::new(
        opts!("http_requests_total_vec", "Total number of HTTP requests made grouped by status code and method."),
        &["status_code", "method"]
    ).unwrap();
}

pub fn register_metrics() {
    REGISTRY.register(Box::new(HTTP_COUNTER.clone())).unwrap();
    REGISTRY.register(Box::new(HTTP_COUNTER_VEC.clone())).unwrap();
}

async fn metrics_handler() -> impl Responder {
    let encoder = TextEncoder::new();
    let metric_families = REGISTRY.gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    HttpResponse::Ok()
        .content_type(encoder.format_type())
        .body(buffer)
}

async fn index() -> HttpResponse {
    HTTP_COUNTER.inc();
    HTTP_COUNTER_VEC.with_label_values(&["200", "GET"]).inc();
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

    register_metrics();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(index))
            .route("/metrics", web::get().to(metrics_handler))
            .configure(controller::transaction_controller::config)
            .wrap_fn(|req, srv| {
                let method = req.method().to_string();
                let fut = srv.call(req);
                async move {
                    let res = fut.await?;
                    let status_code = res.status().as_u16().to_string();
                    HTTP_COUNTER.inc();
                    HTTP_COUNTER_VEC.with_label_values(&[&status_code, &method]).inc();
                    Ok(res)
                }
            })
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
