use std::str::FromStr;
use std::fmt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{self, Decode, Postgres, Type};
use sqlx::postgres::PgTypeInfo;
use sqlx::postgres::PgValueRef;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Transaction {
    pub id: Uuid,
    pub payment_method: PaymentMethod,
    pub status: TransactionStatus,
    pub time: Option<DateTime<Utc>>,
}


#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum PaymentMethod {
    Cash,
    EWallet,
    CreditCard,
}

impl fmt::Display for PaymentMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let method_str = match self {
            PaymentMethod::Cash => "Cash",
            PaymentMethod::EWallet => "E-Wallet",
            PaymentMethod::CreditCard => "Credit Card",
        };
        write!(f, "{}", method_str)
    }
}

impl FromStr for PaymentMethod {
    type Err = ();

    fn from_str(input: &str) -> Result<PaymentMethod, Self::Err> {
        match input {
            "Cash" => Ok(PaymentMethod::Cash),
            "E-Wallet" => Ok(PaymentMethod::EWallet),
            "Credit Card" => Ok(PaymentMethod::CreditCard),
            _ => Err(()),
        }
    }
}

impl<'r> Decode<'r, Postgres> for PaymentMethod {
    fn decode(value: PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as Decode<Postgres>>::decode(value)?;
        s.parse().map_err(|_| "invalid payment method".into())
    }
}

impl Type<Postgres> for PaymentMethod {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("TEXT")
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

impl FromStr for TransactionStatus {
    type Err = ();

    fn from_str(input: &str) -> Result<TransactionStatus, Self::Err> {
        match input {
            "Ordered" => Ok(TransactionStatus::Ordered),
            "Paid" => Ok(TransactionStatus::Paid),
            "Completed" => Ok(TransactionStatus::Completed),
            _ => Err(()),
        }
    }
}

impl<'r> Decode<'r, Postgres> for TransactionStatus {
    fn decode(value: PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as Decode<Postgres>>::decode(value)?;
        s.parse().map_err(|_| "invalid transaction status".into())
    }
}

impl Type<Postgres> for TransactionStatus {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("TEXT")
    }
}
