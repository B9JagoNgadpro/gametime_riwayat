use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::fmt;

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
    pub fn next_step(&mut self) {
        match self.transaction_status {
            TransactionStatus::Ordered => {
                self.transaction_status = TransactionStatus::Paid;
                self.paid_time = Some(Utc::now());
            }
            TransactionStatus::Paid => {
                self.transaction_status = TransactionStatus::Completed;
                self.completed_time = Some(Utc::now());
            }
            _ => {

            }
        }
    }
}