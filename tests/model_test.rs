use gametime_riwayat::model::transaction::{Transaction, PaymentMethod, TransactionStatus};
use rand::Rng;

#[cfg(test)]
mod tests {
    use gametime_riwayat::model::toy::Toy;
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
    async fn test_toy_creation() {
        let mut rng = rand::thread_rng();

        let seller_id = Uuid::new_v4();
        let item_id = Uuid::new_v4();
        let price = rng.gen_range(10000..500000);
        let stock = rng.gen_range(0..1000);
        let sold = rng.gen_range(0..1000);

        let toy = Toy {
            seller_id,
            item_id,
            price,
            stock,
            sold,
        };

        assert_eq!(toy.seller_id, seller_id);
        assert_eq!(toy.item_id, item_id);
        assert_eq!(toy.price, price);
        assert_eq!(toy.stock, stock);
        assert_eq!(toy.sold, sold);
    }
}