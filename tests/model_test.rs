use gametime_riwayat::model::transaction::{Transaction, PaymentMethod, TransactionStatus};
#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use chrono::Utc;
    use std::thread;
    use std::time::Duration;
    use super::*;

    #[actix_web::test]
    async fn test_transaction_creation() {
        let id = Uuid::new_v4();
        let product_id = Uuid::new_v4();
        let seller_id = Uuid::new_v4();
        let buyer_id = Uuid::new_v4();
        let order_time = Utc::now();

        let transaction = Transaction {
            id,
            product_id,
            seller_id,
            buyer_id,
            price: 42069,
            payment_method: PaymentMethod::CreditCard,
            transaction_status: TransactionStatus::Ordered,
            order_time,
            paid_time: None,
            completed_time: None
        };

        assert_eq!(transaction.id, id);
        assert_eq!(transaction.product_id, product_id);
        assert_eq!(transaction.price, 42069);
        assert_eq!(transaction.payment_method, PaymentMethod::CreditCard);

        assert_eq!(transaction.transaction_status, TransactionStatus::Ordered);
        assert_eq!(transaction.order_time, order_time);
        assert!(transaction.paid_time.is_none());
        assert!(transaction.completed_time.is_none());
    }
}
