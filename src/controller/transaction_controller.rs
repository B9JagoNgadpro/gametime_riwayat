use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use crate::service::transaction_service::TransactionService;
use crate::model::transaction::{PaymentMethod, TransactionStatus};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateTransactionDTO {
    pub payment_method: String,
    pub status: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransactionDTO {
    pub id: Uuid,
    pub payment_method: String,
    pub status: String,
    pub time: Option<chrono::DateTime<Utc>>,
}

pub async fn create_transaction(
    pool: web::Data<PgPool>,
    dto: web::Json<CreateTransactionDTO>
) -> impl Responder {
    let service = TransactionService::new(pool.get_ref());
    let payment_method = match dto.payment_method.as_str() {
        "Cash" => PaymentMethod::Cash,
        "EWallet" => PaymentMethod::EWallet,
        "CreditCard" => PaymentMethod::CreditCard,
        _ => return HttpResponse::BadRequest().body("Invalid payment method"),
    };
    let status = match dto.status.as_str() {
        "Ordered" => TransactionStatus::Ordered,
        "Paid" => TransactionStatus::Paid,
        "Completed" => TransactionStatus::Completed,
        _ => return HttpResponse::BadRequest().body("Invalid transaction status"),
    };
    match service.create_transaction(payment_method, status).await {
        Ok(transaction) => HttpResponse::Ok().json(TransactionDTO {
            id: transaction.id,
            payment_method: transaction.payment_method.to_string(),
            status: transaction.status.to_string(),
            time: transaction.time, // Use Option directly
        }),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_transaction(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>
) -> impl Responder {
    let service = TransactionService::new(pool.get_ref());
    match service.get_transaction(path.into_inner()).await {
        Ok(transaction) => HttpResponse::Ok().json(TransactionDTO {
            id: transaction.id,
            payment_method: transaction.payment_method.to_string(),
            status: transaction.status.to_string(),
            time: transaction.time, // Use Option directly
        }),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_transactions_by_buyer(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>
) -> impl Responder {
    let service = TransactionService::new(pool.get_ref());
    match service.get_transactions_by_buyer(&path.into_inner()).await {
        Ok(transactions) => HttpResponse::Ok().json(
            transactions.into_iter().map(|transaction| TransactionDTO {
                id: transaction.id,
                payment_method: transaction.payment_method.to_string(),
                status: transaction.status.to_string(),
                time: transaction.time,
            }).collect::<Vec<_>>()
        ),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_transactions_by_seller(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>
) -> impl Responder {
    let service = TransactionService::new(pool.get_ref());
    match service.get_transactions_by_seller(&path.into_inner()).await {
        Ok(transactions) => HttpResponse::Ok().json(
            transactions.into_iter().map(|transaction| TransactionDTO {
                id: transaction.id,
                payment_method: transaction.payment_method.to_string(),
                status: transaction.status.to_string(),
                time: transaction.time,
            }).collect::<Vec<_>>()
        ),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/transaction", web::post().to(create_transaction))
            .route("/transaction/{id}", web::get().to(get_transaction))
            .route("/transactions/buyer/{buyer_id}", web::get().to(get_transactions_by_buyer))
            .route("/transactions/seller/{seller_id}", web::get().to(get_transactions_by_seller))
    );
}
