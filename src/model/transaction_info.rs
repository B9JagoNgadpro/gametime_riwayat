use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TransactionGameInfo {
    pub transaksi_id: Uuid,
    pub game_nama: String,
    pub game_harga: i64,
    pub tanggal_pembayaran: DateTime<Utc>,
    pub pembeli_id: String,
}
