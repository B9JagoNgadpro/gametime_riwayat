use sqlx::{PgPool, Executor};
use uuid::Uuid;
use chrono::Utc;
use gametime_riwayat::model::{transaksi::Transaksi, game::Game};
use gametime_riwayat::repository::transaksi_repository::TransaksiRepository;

#[cfg(test)]
mod tests {
    use std::env;
    use dotenv::dotenv;    

    use super::*;
    use sqlx::migrate::MigrateDatabase;

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
            penjual_id: "a@gmail.com".to_string(),
        }
    }

    fn valid_transaksi() -> Transaksi {
        Transaksi {
            id: Uuid::new_v4(),
            games: vec![valid_game()],
            total_harga: 5000,
            tanggal_pembayaran: Utc::now(),
            pembeli_id: "a@gmail.com".to_string(),
        }
    }

    #[tokio::test]
    async fn test_create_transaksi() {
        let pool = setup_test_db().await;
        let repo = TransaksiRepository { pool: pool.clone() };
        let mut tx = pool.begin().await.unwrap();

        let transaksi = valid_transaksi();
        let result = repo.create_transaksi(&mut tx, &transaksi).await;
        assert!(result.is_ok());

        tx.commit().await.unwrap();
    }

    #[tokio::test]
    async fn test_create_game() {
        let pool = setup_test_db().await;
        let repo = TransaksiRepository { pool: pool.clone() };
        let mut tx = pool.begin().await.unwrap();

        let game = valid_game();
        let result = repo.create_game(&mut tx, &game).await;
        assert!(result.is_ok());

        tx.commit().await.unwrap();
    }

    #[tokio::test]
    async fn test_associate_game_with_transaksi() {
        let pool = setup_test_db().await;
        let repo = TransaksiRepository { pool: pool.clone() };
        let mut tx = pool.begin().await.unwrap();

        let transaksi = valid_transaksi();
        let game = valid_game();
        repo.create_transaksi(&mut tx, &transaksi).await.unwrap();
        repo.create_game(&mut tx, &game).await.unwrap();

        let result = repo.associate_game_with_transaksi(&mut tx, transaksi.id, game.id).await;
        assert!(result.is_ok());

        tx.commit().await.unwrap();
    }

    #[tokio::test]
    async fn test_get_transactions_by_user() {
        let pool = setup_test_db().await;
        let repo = TransaksiRepository { pool: pool.clone() };
        let mut tx = pool.begin().await.unwrap();

        let transaksi = valid_transaksi();
        repo.create_transaksi(&mut tx, &transaksi).await.unwrap();

        tx.commit().await.unwrap();

        let transactions = repo.get_transactions_by_user(&transaksi.pembeli_id).await.unwrap();
        assert_eq!(transactions.len(), 1);
        assert_eq!(transactions[0], transaksi);
    }

    #[tokio::test]
    async fn test_get_games_by_transaksi() {
        let pool = setup_test_db().await;
        let repo = TransaksiRepository { pool: pool.clone() };
        let mut tx = pool.begin().await.unwrap();

        let transaksi = valid_transaksi();
        let game = valid_game();
        repo.create_transaksi(&mut tx, &transaksi).await.unwrap();
        repo.create_game(&mut tx, &game).await.unwrap();
        repo.associate_game_with_transaksi(&mut tx, transaksi.id, game.id).await.unwrap();

        tx.commit().await.unwrap();

        let games = repo.get_games_by_transaksi(transaksi.id).await.unwrap();
        assert_eq!(games.len(), 1);
        assert_eq!(games[0], game);
    }

    #[tokio::test]
    async fn test_get_transaction_game_info_by_penjual() {
        let pool = setup_test_db().await;
        let repo = TransaksiRepository { pool: pool.clone() };

        let mut tx = pool.begin().await.unwrap();

        let transaksi = valid_transaksi();
        repo.create_transaksi(&mut tx, &transaksi).await.unwrap();
        for game in &transaksi.games {
            repo.create_game(&mut tx, game).await.unwrap();
            repo.associate_game_with_transaksi(&mut tx, transaksi.id, game.id).await.unwrap();
        }

        tx.commit().await.unwrap();

        let results = repo.get_transaction_game_info_by_penjual("a@gmail.com").await.unwrap();
        assert_eq!(results.len(), 1);
        let result = &results[0];
        assert_eq!(result.transaksi_id, transaksi.id);
        assert_eq!(result.game_nama, transaksi.games[0].nama);
        assert_eq!(result.game_harga, transaksi.games[0].harga);
        assert_eq!(result.tanggal_pembayaran, transaksi.tanggal_pembayaran);
        assert_eq!(result.pembeli_id, transaksi.pembeli_id);
    }

    #[tokio::test]
    async fn test_get_transaction_game_info_by_penjual_no_results() {
        let pool = setup_test_db().await;
        let repo = TransaksiRepository { pool: pool.clone() };

        let results = repo.get_transaction_game_info_by_penjual("nonexistent@gmail.com").await.unwrap();
        assert!(results.is_empty());
    }
}
