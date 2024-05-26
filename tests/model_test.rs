#[cfg(test)]
mod tests {
    use chrono::Utc;
    use gametime_riwayat::model::{game::Game, transaksi::Transaksi};
    use serde_json::json;
    use uuid::Uuid;

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

    #[test]
    fn test_game_serialization() {
        let game = valid_game();
        let serialized_game = serde_json::to_string(&game).unwrap();
        let deserialized_game: Game = serde_json::from_str(&serialized_game).unwrap();
        assert_eq!(game, deserialized_game);
    }

    #[test]
    fn test_game_deserialization_valid() {
        let game = valid_game();
        let game_json = json!({
            "id": game.id,
            "nama": game.nama,
            "deskripsi": game.deskripsi,
            "harga": game.harga,
            "kategori": game.kategori,
        });

        let deserialized_game: Game = serde_json::from_value(game_json).unwrap();
        assert_eq!(game, deserialized_game);
    }

    #[test]
    fn test_game_deserialization_invalid_missing_field() {
        let game_json = json!({
            "id": Uuid::new_v4(),
            "nama": "Test Game",
            "deskripsi": "A test game",
            "harga": 5000,
            "kategori": "Action"
            // Missing "penjual_id"
        });

        let deserialized_game: Result<Game, _> = serde_json::from_value(game_json);
        assert!(deserialized_game.is_err());
    }

    #[test]
    fn test_game_deserialization_invalid_type() {
        let game_json = json!({
            "id": Uuid::new_v4(),
            "nama": "Test Game",
            "deskripsi": "A test game",
            "harga": "5000",  // Invalid type, should be i64
            "kategori": "Action",
            "penjual_id": Uuid::new_v4()
        });

        let deserialized_game: Result<Game, _> = serde_json::from_value(game_json);
        assert!(deserialized_game.is_err());
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

    #[test]
    fn test_transaksi_serialization() {
        let transaksi = valid_transaksi();
        let serialized_transaksi = serde_json::to_string(&transaksi).unwrap();
        let deserialized_transaksi: Transaksi = serde_json::from_str(&serialized_transaksi).unwrap();
        assert_eq!(transaksi, deserialized_transaksi);
    }

    #[test]
    fn test_transaksi_deserialization_valid() {
        let transaksi = valid_transaksi();
        let transaksi_json = json!({
            "id": transaksi.id,
            "games": [{
                "id": transaksi.games[0].id,
                "nama": transaksi.games[0].nama,
                "deskripsi": transaksi.games[0].deskripsi,
                "harga": transaksi.games[0].harga,
                "kategori": transaksi.games[0].kategori,
            }],
            "total_harga": transaksi.total_harga,
            "tanggal_pembayaran": transaksi.tanggal_pembayaran,
            "pembeli_id": transaksi.pembeli_id
        });

        let deserialized_transaksi: Transaksi = serde_json::from_value(transaksi_json).unwrap();
        assert_eq!(transaksi, deserialized_transaksi);
    }

    #[test]
    fn test_transaksi_deserialization_invalid_missing_field() {
        let transaksi_json = json!({
            "id": Uuid::new_v4(),
            "games": [{
                "id": Uuid::new_v4(),
                "nama": "Test Game",
                "deskripsi": "A test game",
                "harga": 5000,
                "kategori": "Action",
                "penjual_id": Uuid::new_v4()
            }],
            "total_harga": 5000,
            "status_pembayaran": "Paid",
            "tanggal_pembayaran": Utc::now()
            // Missing "pembeli_id"
        });

        let deserialized_transaksi: Result<Transaksi, _> = serde_json::from_value(transaksi_json);
        assert!(deserialized_transaksi.is_err());
    }

    #[test]
    fn test_transaksi_deserialization_invalid_type() {
        let transaksi_json = json!({
            "id": Uuid::new_v4(),
            "games": [{
                "id": Uuid::new_v4(),
                "nama": "Test Game",
                "deskripsi": "A test game",
                "harga": 5000,
                "kategori": "Action",
                "penjual_id": Uuid::new_v4()
            }],
            "total_harga": "5000",  // Invalid type, should be i64
            "status_pembayaran": "Paid",
            "tanggal_pembayaran": Utc::now(),
            "pembeli_id": Uuid::new_v4()
        });

        let deserialized_transaksi: Result<Transaksi, _> = serde_json::from_value(transaksi_json);
        assert!(deserialized_transaksi.is_err());
    }
}
