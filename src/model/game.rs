use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Game {
    pub id: Uuid,
    pub nama: String,
    pub deskripsi: String,
    pub harga: i64,
    pub kategori: String,
    pub penjual_id: String,
}