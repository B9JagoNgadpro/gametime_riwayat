use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::model::{game::Game, transaction::{PaymentMethod, Transaction, TransactionStatus}};

pub struct TransactionRepository {
    pool: Pool<Postgres>
}

impl TransactionRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn add_transaction(&self, transaction: &Transaction) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO transactions (id, payment_method, status, time)
            VALUES ($1, $2, $3, $4)
            "#,
            transaction.id,
            transaction.payment_method as PaymentMethod,
            transaction.status as TransactionStatus,
            transaction.time
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn add_transaction_game(&self, game: &Game) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO games (id, transaction_id, seller_id, buyer_id, amount)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            game.id,
            game.transaction_id,
            game.seller_id,
            game.buyer_id,
            game.amount
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_transactions_buyer(&self, buyer_id: &str) -> Result<(), sqlx::Error> {
        let transactions = sqlx::query_as!(
            Transaction,
            r#"
            SELECT t.id, t.payment_method as "payment_method: _", t.status as "status: _", t.time
            FROM transactions AS t
            JOIN games AS g ON t.id = g.transaction_id
            WHERE g.buyer_id = $1
            "#,
            buyer_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(transactions)
    }

    pub async fn get_transactions_seller(&self, seller_id: &str) -> Result<(), sqlx::Error> {
        let transactions = sqlx::query_as!(
            Transaction,
            r#"
            SELECT t.id, t.payment_method as "payment_method: _", t.status as "status: _", t.time
            FROM transactions AS t
            JOIN games AS g ON t.id = g.transaction_id
            WHERE g.buyer_id = $1
            "#,
            seller_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(transactions)
    }
}