#[cfg(test)]
mod tests {
    use super::*;
    use gametime_riwayat::model::game::Game;
    use serde_json::json;
    use uuid::Uuid;

    fn valid_game() -> Game {
        Game {
            id: Uuid::new_v4(),
            nama: "Test Game".to_string(),
            deskripsi: "A test game".to_string(),
            harga: 5000,
            kategori: "Action".to_string(),
            penjual_id: Uuid::new_v4(),
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
            "penjual_id": game.penjual_id
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
}
