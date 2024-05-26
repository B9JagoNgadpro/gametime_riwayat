use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::model::game::Game;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Transaksi {
    pub id: Uuid,
    pub games: Vec<Game>,
    pub total_harga: i64,
    pub tanggal_pembayaran: DateTime<Utc>,
    pub pembeli_id: Uuid,
}