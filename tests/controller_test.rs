use actix_web::{test, web, App};
use uuid::Uuid;
use chrono::Utc;
use gametime_riwayat::model::{transaksi::Transaksi, game::Game, response::Response};
use gametime_riwayat::repository::transaksi_repository::TransaksiRepository;
use gametime_riwayat::service::transaksi_service::TransaksiService;
use gametime_riwayat::controller::transaksi_controller::{create_transaksi, get_user_transactions};

#[cfg(test)]
mod tests {
    use super::*;
    use gametime_riwayat::{controller::transaksi_controller::get_transaction_game_info_by_penjual, model::transaction_info::TransactionGameInfo};
    use sqlx::{migrate::MigrateDatabase, Executor, PgPool};

    async fn setup_test_db() -> PgPool {
        let database_url = "postgres://user:password@localhost/test_db";

        // Create the database if it doesn't exist
        if !sqlx::Postgres::database_exists(database_url).await.unwrap_or(false) {
            sqlx::Postgres::create_database(database_url).await.unwrap();
        }

        let pool = PgPool::connect(database_url).await.unwrap();

        pool.execute(
            r#"
            CREATE TABLE IF NOT EXISTS transaksi (
                id UUID PRIMARY KEY,
                total_harga BIGINT NOT NULL,
                status_pembayaran VARCHAR NOT NULL,
                tanggal_pembayaran TIMESTAMPTZ NOT NULL,
                pembeli_id VARCHAR NOT NULL
            );
            CREATE TABLE IF NOT EXISTS game (
                id UUID PRIMARY KEY,
                nama VARCHAR NOT NULL,
                deskripsi TEXT NOT NULL,
                harga BIGINT NOT NULL,
                kategori VARCHAR NOT NULL,
                penjual_id UUID NOT NULL
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
            pembeli_id: "user@example.com".to_string(),
        }
    }

    #[actix_web::test]
    async fn test_create_transaksi() {
        let pool = setup_test_db().await;
        let repo = TransaksiRepository { pool: pool.clone() };
        let service = web::Data::new(TransaksiService::new(repo));

        let mut app = test::init_service(
            App::new()
                .app_data(service.clone())
                .route("/create", web::post().to(create_transaksi))
        )
        .await;

        let transaksi = valid_transaksi();
        let req = test::TestRequest::post()
            .uri("/create")
            .set_json(&transaksi)
            .to_request();

        let resp: Response = test::call_and_read_body_json(&mut app, req).await;

        assert_eq!(resp.message, "Created a transaksi with games.".to_string());
    }

    #[actix_web::test]
    async fn test_create_transaksi_invalid() {
        let pool = setup_test_db().await;
        let repo = TransaksiRepository { pool: pool.clone() };
        let service = web::Data::new(TransaksiService::new(repo));

        let mut app = test::init_service(
            App::new()
                .app_data(service.clone())
                .route("/create", web::post().to(create_transaksi))
        )
        .await;

        let mut transaksi = valid_transaksi();
        transaksi.total_harga = -1; // Invalid total_harga
        let req = test::TestRequest::post()
            .uri("/create")
            .set_json(&transaksi)
            .to_request();

        let resp: Response = test::call_and_read_body_json(&mut app, req).await;

        assert!(resp.message.contains("Couldn't create a new transaksi."));
    }

    #[actix_web::test]
    async fn test_get_user_transactions() {
        let pool = setup_test_db().await;
        let repo = TransaksiRepository { pool: pool.clone() };
        let service = web::Data::new(TransaksiService::new(repo));

        let mut app = test::init_service(
            App::new()
                .app_data(service.clone())
                .route("/get/{user_id}", web::get().to(get_user_transactions))
        )
        .await;

        let transaksi = valid_transaksi();
        service.create_transaksi(transaksi.clone()).await.unwrap();

        let req = test::TestRequest::get()
            .uri(&format!("/get/{}", transaksi.pembeli_id))
            .to_request();

        let resp: Vec<Transaksi> = test::call_and_read_body_json(&mut app, req).await;

        assert_eq!(resp.len(), 1);
        assert_eq!(resp[0].id, transaksi.id);
        assert_eq!(resp[0].games.len(), 1);
        assert_eq!(resp[0].games[0].id, transaksi.games[0].id);
    }

    #[actix_web::test]
    async fn test_get_user_transactions_invalid_user() {
        let pool = setup_test_db().await;
        let repo = TransaksiRepository { pool: pool.clone() };
        let service = web::Data::new(TransaksiService::new(repo));

        let mut app = test::init_service(
            App::new()
                .app_data(service.clone())
                .route("/get/{user_id}", web::get().to(get_user_transactions))
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/get/invalid@example.com")
            .to_request();

        let resp: Vec<Transaksi> = test::call_and_read_body_json(&mut app, req).await;

        assert!(resp.is_empty());
    }

    #[actix_web::test]
    async fn test_get_transaction_game_info_by_penjual() {
        let pool = setup_test_db().await;
        let repo = TransaksiRepository { pool: pool.clone() };
        let service = web::Data::new(TransaksiService::new(repo));

        let mut app = test::init_service(
            App::new()
                .app_data(service.clone())
                .route("/get_transaction_game_info_by_penjual/{penjual_id}", web::get().to(get_transaction_game_info_by_penjual))
        )
        .await;

        let mut tx = pool.begin().await.unwrap();

        let transaksi = valid_transaksi();
        service.repository.create_transaksi(&mut tx, &transaksi).await.unwrap();
        for game in &transaksi.games {
            service.repository.create_game(&mut tx, game).await.unwrap();
            service.repository.associate_game_with_transaksi(&mut tx, transaksi.id, game.id).await.unwrap();
        }

        tx.commit().await.unwrap();

        let req = test::TestRequest::get()
            .uri("/get_transaction_game_info_by_penjual/a@gmail.com")
            .to_request();

        let resp: Vec<TransactionGameInfo> = test::call_and_read_body_json(&mut app, req).await;

        assert_eq!(resp.len(), 1);
        let result = &resp[0];
        assert_eq!(result.transaksi_id, transaksi.id);
        assert_eq!(result.game_nama, transaksi.games[0].nama);
        assert_eq!(result.game_harga, transaksi.games[0].harga);
        assert_eq!(result.tanggal_pembayaran, transaksi.tanggal_pembayaran);
        assert_eq!(result.pembeli_id, transaksi.pembeli_id);
    }

    #[actix_web::test]
    async fn test_get_transaction_game_info_by_penjual_no_results() {
        let pool = setup_test_db().await;
        let repo = TransaksiRepository { pool: pool.clone() };
        let service = web::Data::new(TransaksiService::new(repo));

        let mut app = test::init_service(
            App::new()
                .app_data(service.clone())
                .route("/get_transaction_game_info_by_penjual/{penjual_id}", web::get().to(get_transaction_game_info_by_penjual))
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/get_transaction_game_info_by_penjual/nonexistent@gmail.com")
            .to_request();

        let resp: Vec<TransactionGameInfo> = test::call_and_read_body_json(&mut app, req).await;

        assert!(resp.is_empty());
    }

}
