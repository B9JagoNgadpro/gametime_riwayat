use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::model::game::Game;

pub struct Transaksi {
    id: Uuid,
    games: Vec<Game>,
    total_harga: u64,
    status_pembayaran: String,
    tanggal_pembayaran: DateTime<Utc>,
    pembeli_id: Uuid,
}