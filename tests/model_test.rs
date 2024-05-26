#[cfg(test)]
mod tests {
    use super::*;
    use gametime_riwayat::model::game::Game;
    use serde_json::json;
    use uuid::Uuid;

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
