use crate::repository::transaction_repository::TransactionRepository;
use crate::model::transaction::{Transaction, PaymentMethod, TransactionStatus};
use uuid::Uuid;
use sqlx::PgPool;
use chrono::Utc;

pub struct TransactionService<'a> {
    repository: TransactionRepository<'a>,
}

impl<'a> TransactionService<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self {
            repository: TransactionRepository::new(pool),
        }
    }

    pub async fn create_transaction(&self, payment_method: PaymentMethod, status: TransactionStatus) -> Result<Transaction, sqlx::Error> {
        let transaction = Transaction {
            id: Uuid::new_v4(),
            payment_method,
            status,
            time: Some(Utc::now()), // Setting the current time as an Option
        };
        self.repository.add_transaction(&transaction).await?;
        Ok(transaction)
    }

    pub async fn get_transaction(&self, id: Uuid) -> Result<Transaction, sqlx::Error> {
        self.repository.find_by_id(id).await
    }

    pub async fn get_transactions_by_buyer(&self, buyer_id: &Uuid) -> Result<Vec<Transaction>, sqlx::Error> {
        self.repository.get_transactions_buyer(buyer_id).await
    }

    pub async fn get_transactions_by_seller(&self, seller_id: &Uuid) -> Result<Vec<Transaction>, sqlx::Error> {
        self.repository.get_transactions_seller(seller_id).await
    }
}
