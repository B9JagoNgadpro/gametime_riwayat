use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::fmt;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Transaction {
    pub id: Uuid,
    pub payment_method: PaymentMethod,
    pub status: TransactionStatus,
    pub time: DateTime<Utc>,
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
            payment_method: PaymentMethod::EWallet,
            status: TransactionStatus::Ordered,
            time: Utc::now(),
        }
    }
}