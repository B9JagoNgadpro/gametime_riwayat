use gametime_riwayat::model::transaction::{Transaction, PaymentMethod, TransactionStatus};
use rand::Rng;

#[cfg(test)]
mod tests {
    use gametime_riwayat::model::game::Game;
    use uuid::Uuid;
    use chrono::Utc;
    use super::*;

    #[actix_web::test]
    async fn test_transaction_creation() {
        let id = Uuid::new_v4();
        let time = Utc::now();

        let transaction = Transaction {
            id,
            payment_method: PaymentMethod::CreditCard,
            status: TransactionStatus::Ordered,
            time,
        };

        assert_eq!(transaction.id, id);
        assert_eq!(transaction.payment_method, PaymentMethod::CreditCard);

        assert_eq!(transaction.status, TransactionStatus::Ordered);
        assert_eq!(transaction.time, time);
    }

    #[actix_web::test]
    async fn test_game_creation() {
        let mut rng = rand::thread_rng();

        let id = Uuid::new_v4();
        let transaction_id = Uuid::new_v4();
        let seller_id = Uuid::new_v4();
        let buyer_id = Uuid::new_v4();
        let amount = rng.gen_range(1..200);

        let toy = Game {
            id,
            transaction_id,
            seller_id,
            buyer_id,
            amount
        };

        assert_eq!(toy.id, id);
        assert_eq!(toy.transaction_id, transaction_id);
        assert_eq!(toy.seller_id, seller_id);
        assert_eq!(toy.buyer_id, buyer_id);
    }
}