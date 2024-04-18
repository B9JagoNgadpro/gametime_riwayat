use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::fmt;
use rand::Rng;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Transaction {
    pub id: Uuid,
    pub product_id: Uuid,
    pub seller_id: Uuid,
    pub buyer_id: Uuid,
    pub price: u64,
    pub payment_method: PaymentMethod,
    pub transaction_status: TransactionStatus,
    pub order_time: DateTime<Utc>,
    pub paid_time: Option<DateTime<Utc>>,
    pub completed_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum PaymentMethod {
    Cash,
    EWallet,
    CreditCard,
}

impl fmt::Display for PaymentMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_str = match self {
            PaymentMethod::Cash => "Cash",
            PaymentMethod::EWallet => "E-Wallet",
            PaymentMethod::CreditCard => "Credit Card",
        };
        write!(f, "{}", status_str)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum TransactionStatus {
    Ordered,
    Paid,
    Completed,
}

impl fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_str = match self {
            TransactionStatus::Ordered => "Ordered",
            TransactionStatus::Paid => "Paid",
            TransactionStatus::Completed => "Completed",
        };
        write!(f, "{}", status_str)
    }
}

impl Transaction {
    pub fn mock() -> Self {
        Self {
            id: Uuid::new_v4(),
            product_id: Uuid::new_v4(),
            seller_id: Uuid::new_v4(),
            buyer_id: Uuid::new_v4(),
            price: rand::thread_rng().gen_range(50..500),
            payment_method: PaymentMethod::EWallet,
            transaction_status: TransactionStatus::Ordered,
            order_time: Utc::now(),
            paid_time: None,
            completed_time: None,
        }
    }
}