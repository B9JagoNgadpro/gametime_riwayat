use uuid::Uuid;
use crate::model::transaksi::Transaksi;
use crate::repository::transaksi_repository::TransaksiRepository;

pub struct TransaksiService {
    repository: TransaksiRepository,
}

impl TransaksiService {
    pub fn new(repository: TransaksiRepository) -> Self {
        Self { repository }
    }

    pub async fn create_transaksi(&self, transaksi: Transaksi) -> Result<(), String> {
        let mut tx = self.repository.pool.begin().await.map_err(|e| e.to_string())?;
        
        self.repository.create_transaksi(&mut tx, &transaksi)
            .await.map_err(|e| e.to_string())?;

        for game in &transaksi.games {
            self.repository.create_game(&mut tx, game)
                .await.map_err(|e| e.to_string())?;
            self.repository.associate_game_with_transaksi(&mut tx, transaksi.id, game.id)
                .await.map_err(|e| e.to_string())?;
        }

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn get_user_transactions(&self, user_id: Uuid) -> Result<Vec<Transaksi>, String> {
        let mut transactions = self.repository.get_transactions_by_user(user_id)
            .await.map_err(|e| e.to_string())?;
        
        for transaction in &mut transactions {
            let games = self.repository.get_games_by_transaksi(transaction.id)
                .await.map_err(|e| e.to_string())?;
            transaction.games = games;
        }
        
        Ok(transactions)
    }
}
