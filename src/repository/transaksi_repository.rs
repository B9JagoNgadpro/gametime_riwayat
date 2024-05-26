use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;
use crate::model::{transaksi::Transaksi, game::Game};

pub struct TransaksiRepository {
    pub pool: PgPool,
}

impl TransaksiRepository {
    pub async fn create_transaksi(
        &self, 
        tx: &mut Transaction<'_, Postgres>,
        transaksi: &Transaksi
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO transaksi (id, total_harga, tanggal_pembayaran, pembeli_id)
             VALUES ($1, $2, $3, $4)",
            transaksi.id,
            transaksi.total_harga as i64,
            transaksi.tanggal_pembayaran,
            transaksi.pembeli_id
        )
        .execute(&mut **tx)
        .await?;
        
        Ok(())
    }

    pub async fn create_game(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        game: &Game
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO game (id, nama, deskripsi, harga, kategori)
             VALUES ($1, $2, $3, $4, $5)
             ON CONFLICT (id) DO NOTHING",
            game.id,
            game.nama,
            game.deskripsi,
            game.harga as i64,
            game.kategori,
        )
        .execute(&mut **tx)
        .await?;
        
        Ok(())
    }

    pub async fn associate_game_with_transaksi(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        transaksi_id: Uuid,
        game_id: Uuid
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO transaksi_game (transaksi_id, game_id)
             VALUES ($1, $2)",
            transaksi_id,
            game_id
        )
        .execute(&mut **tx)
        .await?;
        
        Ok(())
    }

    pub async fn get_transactions_by_user(
        &self,
        user_id: Uuid
    ) -> Result<Vec<Transaksi>, sqlx::Error> {
        let transactions = sqlx::query!(
            r#"
            SELECT id, total_harga, status_pembayaran, tanggal_pembayaran, pembeli_id
            FROM transaksi
            WHERE pembeli_id = $1
            ORDER BY tanggal_pembayaran DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|record| Transaksi {
            id: record.id,
            games: Vec::new(), // Will be populated later
            total_harga: record.total_harga,
            tanggal_pembayaran: record.tanggal_pembayaran,
            pembeli_id: record.pembeli_id,
        })
        .collect();
        
        Ok(transactions)
    }

    pub async fn get_games_by_transaksi(
        &self,
        transaksi_id: Uuid
    ) -> Result<Vec<Game>, sqlx::Error> {
        let games = sqlx::query_as!(
            Game,
            r#"
            SELECT g.id, g.nama, g.deskripsi, g.harga, g.kategori
            FROM game g
            JOIN transaksi_game tg ON g.id = tg.game_id
            WHERE tg.transaksi_id = $1
            "#,
            transaksi_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(games)
    }
}
