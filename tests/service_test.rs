use sqlx::{PgPool, Executor};
use uuid::Uuid;
use chrono::Utc;
use gametime_riwayat::model::{transaksi::Transaksi, game::Game};
use gametime_riwayat::repository::transaksi_repository::TransaksiRepository;
use gametime_riwayat::service::transaksi_service::TransaksiService;

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::migrate::MigrateDatabase;

    use std::env;
    use dotenv::dotenv; 

    async fn setup_test_db() -> PgPool {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("Missing database URL");

        // Create the database if it doesn't exist
        if !sqlx::Postgres::database_exists(&database_url).await.unwrap_or(false) {
            sqlx::Postgres::create_database(&database_url).await.unwrap();
        }

        let pool = PgPool::connect(&database_url).await.unwrap();

        pool.execute(
            r#"
            CREATE TABLE IF NOT EXISTS transaksi (
                id UUID PRIMARY KEY,
                total_harga BIGINT NOT NULL,
                tanggal_pembayaran TIMESTAMPTZ NOT NULL,
                pembeli_id UUID NOT NULL
            );
            CREATE TABLE IF NOT EXISTS game (
                id UUID PRIMARY KEY,
                nama VARCHAR NOT NULL,
                deskripsi TEXT NOT NULL,
                harga BIGINT NOT NULL,
                kategori VARCHAR NOT NULL
            );
            CREATE TABLE IF NOT EXISTS transaksi_game (
                transaksi_id UUID NOT NULL,
                game_id UUID NOT NULL,
                PRIMARY KEY (transaksi_id, game_id),
                FOREIGN KEY (transaksi_id) REFERENCES transaksi (id) ON DELETE CASCADE,
                FOREIGN KEY (game_id) REFERENCES game (id) ON DELETE CASCADE
            );
            "#,
        )
        .await
        .unwrap();

        pool
    }

    fn valid_game() -> Game {
        Game {
            id: Uuid::new_v4(),
            nama: "Test Game".to_string(),
            deskripsi: "A test game".to_string(),
            harga: 5000,
            kategori: "Action".to_string(),
        }
    }

    fn valid_transaksi() -> Transaksi {
        Transaksi {
            id: Uuid::new_v4(),
            games: vec![valid_game()],
            total_harga: 5000,
            tanggal_pembayaran: Utc::now(),
            pembeli_id: Uuid::new_v4(),
        }
    }

    #[tokio::test]
    async fn test_create_transaksi() {
        let pool = setup_test_db().await;
        let repo = TransaksiRepository { pool: pool.clone() };
        let service = TransaksiService::new(repo);

        let transaksi = valid_transaksi();
        let result = service.create_transaksi(transaksi).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_user_transactions() {
        let pool = setup_test_db().await;
        let repo = TransaksiRepository { pool: pool.clone() };
        let service = TransaksiService::new(repo);

        let transaksi = valid_transaksi();
        service.create_transaksi(transaksi.clone()).await.unwrap();

        let transactions = service.get_user_transactions(transaksi.pembeli_id).await.unwrap();
        assert_eq!(transactions.len(), 1);
        assert_eq!(transactions[0].id, transaksi.id);
        assert_eq!(transactions[0].games.len(), 1);
        assert_eq!(transactions[0].games[0].id, transaksi.games[0].id);
    }
}
