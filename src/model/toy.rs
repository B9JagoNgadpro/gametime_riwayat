use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Toy {
    pub item_id: Uuid,
    pub seller_id: Uuid,
    pub price: i32,
    pub stock: i32,
    pub sold: i32
}