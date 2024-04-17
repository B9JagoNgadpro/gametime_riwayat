use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Transaction {
    pub id: Uuid,
    pub product_id: Uuid,
    pub seller_id: Uuid,
    pub buyer_id: Uuid,
    pub price: u64,
    pub payment: String,
    pub status: String,
    pub order_time: DateTime<Utc>,
    pub paid_time: Option<DateTime<Utc>>,
    pub completed_time: Option<DateTime<Utc>>,
}

impl Transaction {
    pub fn next_step(&mut self) {
        match self.status.as_str() {
            "Ordered" => {
                self.status = "Paid".to_string();
                self.paid_time = Some(Utc::now());
            }
            "Paid" => {
                self.status = "Completed".to_string();
                self.completed_time = Some(Utc::now());
            }
            _ => {

            }
        }
    }
}