use actix_web::{web, HttpResponse};
use crate::model::{transaksi::Transaksi, response::Response};
use crate::service::transaksi_service::TransaksiService;
use crate::adapter::redis_adapter::RedisAdapter;

pub async fn create_transaksi(
    body: web::Json<Transaksi>,
    service: web::Data<TransaksiService<RedisAdapter>>,
) -> HttpResponse {
    match service.create_transaksi(body.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(Response {
            message: "Created a transaksi with games.".to_string(),
        }),
        Err(err) => HttpResponse::InternalServerError().json(Response {
            message: err,
        }),
    }
}

pub async fn get_user_transactions(
    user_id: web::Path<String>,
    service: web::Data<TransaksiService<RedisAdapter>>,
) -> HttpResponse {
    match service.get_user_transactions(&user_id).await {
        Ok(transactions) => HttpResponse::Ok().json(transactions),
        Err(err) => HttpResponse::InternalServerError().json(Response {
            message: err,
        }),
    }
}

pub async fn get_transaction_game_info_by_penjual(
    penjual_id: web::Path<String>,
    service: web::Data<TransaksiService<RedisAdapter>>,
) -> HttpResponse {
    match service.get_transaction_game_info_by_penjual(&penjual_id).await {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(err) => HttpResponse::InternalServerError().json(Response {
            message: err,
        }),
    }
}
