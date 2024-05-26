use uuid::Uuid;

pub struct Game {
    id: Uuid,
    nama: String,
    deskripsi: String,
    harga: u64,
    kategori: String,
    stok: u64,
    penjual_id: Uuid,
}