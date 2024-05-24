use sqlx::{Pool, Postgres};
use uuid::Uuid;
use crate::model::{game::Game, transaction::Transaction};

pub struct TransactionRepository<'a> {
    pool: &'a Pool<Postgres>,
}

impl<'a> TransactionRepository<'a> {
    pub fn new(pool: &'a Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn add_transaction(&self, transaction: &Transaction) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO transactions (id, payment_method, status, time)
            VALUES ($1, $2, $3, $4)
            "#,
            transaction.id,
            transaction.payment_method.to_string(),
            transaction.status.to_string(),
            transaction.time
        )
        .execute(self.pool)
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
        .execute(self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_transactions_buyer(&self, buyer_id: &Uuid) -> Result<Vec<Transaction>, sqlx::Error> {
        let rows = sqlx::query!(
            r#"
            SELECT t.id, t.payment_method, t.status, t.time
            FROM transactions AS t
            JOIN games AS g ON t.id = g.transaction_id
            WHERE g.buyer_id = $1
            "#,
            buyer_id
        )
        .fetch_all(self.pool)
        .await?;

        let transactions: Vec<Transaction> = rows.into_iter().map(|row| {
            Transaction {
                id: row.id,
                payment_method: row.payment_method.parse().unwrap(),
                status: row.status.parse().unwrap(),
                time: row.time,
            }
        }).collect();

        Ok(transactions)
    }

    pub async fn get_transactions_seller(&self, seller_id: &Uuid) -> Result<Vec<Transaction>, sqlx::Error> {
        let rows = sqlx::query!(
            r#"
            SELECT t.id, t.payment_method, t.status, t.time
            FROM transactions AS t
            JOIN games AS g ON t.id = g.transaction_id
            WHERE g.seller_id = $1
            "#,
            seller_id
        )
        .fetch_all(self.pool)
        .await?;

        let transactions: Vec<Transaction> = rows.into_iter().map(|row| {
            Transaction {
                id: row.id,
                payment_method: row.payment_method.parse().unwrap(),
                status: row.status.parse().unwrap(),
                time: row.time,
            }
        }).collect();

        Ok(transactions)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Transaction, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            SELECT id, payment_method, status, time
            FROM transactions
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(self.pool)
        .await?;

        let transaction = Transaction {
            id: row.id,
            payment_method: row.payment_method.parse().unwrap(),
            status: row.status.parse().unwrap(),
            time: row.time,
        };

        Ok(transaction)
    }
}
