use gametime_riwayat::model::transaction::{Transaction, PaymentMethod, TransactionStatus};
#[cfg(test)]
mod tests {
    use gametime_riwayat::model::history::HistoryBuilder;
    use uuid::Uuid;
    use chrono::Utc;
    use super::*;

    #[actix_web::test]
    async fn test_transaction_creation() {
        let product_id = Uuid::new_v4();
        let seller_id = Uuid::new_v4();
        let buyer_id = Uuid::new_v4();
        let time = Utc::now();

        let transaction = Transaction {
            product_id,
            seller_id,
            buyer_id,
            payment_method: PaymentMethod::CreditCard,
            status: TransactionStatus::Ordered,
            time,
        };

        assert_eq!(transaction.product_id, product_id);
        assert_eq!(transaction.payment_method, PaymentMethod::CreditCard);

        assert_eq!(transaction.status, TransactionStatus::Ordered);
        assert_eq!(transaction.time, time);
    }

    #[actix_web::test]
    async fn test_history_initiate() {
        let transactions = vec![
            Transaction::mock(),
            Transaction::mock(),
            Transaction::mock(),
        ];

        let mut builder = HistoryBuilder::new();
        builder.transactions.extend(transactions.clone());

        let history = builder.build();

        assert_eq!(history.transactions, transactions);
    }
}