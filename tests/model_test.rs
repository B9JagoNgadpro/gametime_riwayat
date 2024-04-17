#[cfg(test)]
mod tests {
    use actix_web::test;
    use uuid::Uuid;
    use chrono::{DateTime, Utc};
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
            payment: "Credit Card".to_string(),
            status: "Ordered".to_string(),
            order_time,
            paid_time: None,
            completed_time: None
        };

        assert_eq!(transaction.id, id);
        assert_eq!(transaction.product_id, product_id);
        assert_eq!(transaction.price, 42069);
        assert_eq!(transaction.payment, "Credit Card".to_string());

        assert_eq!(transaction.status, "Ordered".to_string());
        assert_eq!(transaction.order_time, order_time);
        assert!(transaction.paid_time.is_none());
        assert!(transaction.completed_time.is_none());
    }

    #[actix_web::test]
    async fn test_transaction_process() {
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
            status: "Ordered".to_string(),
            payment: "Credit Card".to_string(),
            order_time,
            paid_time: None,
            completed_time: None
        };

        assert_eq!(transaction.id, id);
        assert_eq!(transaction.product_id, product_id);
        assert_eq!(transaction.price, 42069);
        assert_eq!(transaction.payment, "Credit Card".to_string());
        
        assert_eq!(transaction.status, "Ordered".to_string());
        assert_eq!(transaction.order_time, order_time);
        assert!(transaction.paid_time.is_none());
        assert!(transaction.completed_time.is_none());

        thread::sleep(Duration::from_secs(1));
        transaction.next_step();
        assert_eq!(transaction.status, "Paid".to_string());
        assert!(!transaction.paid_time.is_none());

        thread::sleep(Duration::from_secs(1));
        transaction.next_step();
        assert_eq!(transaction.status, "Completed".to_string());
        assert!(!transaction.completed_time.is_none());
    }
}
