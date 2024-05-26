use crate::repository::transaksi_repository::TransaksiRepository;
use crate::model::transaksi::Transaksi;
use crate::model::transaction_info::TransactionGameInfo;
use crate::adapter::cache::Cache;

pub struct TransaksiService<C: Cache + Send + Sync> {
    repository: TransaksiRepository,
    cache: C,
}

impl<C: Cache + Send + Sync> TransaksiService<C> {
    pub fn new(repository: TransaksiRepository, cache: C) -> Self {
        Self { repository, cache }
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

    pub async fn get_user_transactions(&self, user_id: &str) -> Result<Vec<Transaksi>, String> {
        let cache_key = format!("user_transactions:{}", user_id);
        if let Some(cached) = self.cache.get(&cache_key).await {
            let transactions: Vec<Transaksi> = serde_json::from_str(&cached).unwrap();
            return Ok(transactions);
        }

        let mut transactions = self.repository.get_transactions_by_user(user_id)
            .await.map_err(|e| e.to_string())?;
        
        for transaction in &mut transactions {
            let games = self.repository.get_games_by_transaksi(transaction.id)
                .await.map_err(|e| e.to_string())?;
            transaction.games = games;
        }

        let serialized = serde_json::to_string(&transactions).unwrap();
        self.cache.set(&cache_key, &serialized).await;

        Ok(transactions)
    }

    pub async fn get_transaction_game_info_by_penjual(&self, penjual_id: &str) -> Result<Vec<TransactionGameInfo>, String> {
        self.repository.get_transaction_game_info_by_penjual(penjual_id)
            .await.map_err(|e| e.to_string())
    }
}
