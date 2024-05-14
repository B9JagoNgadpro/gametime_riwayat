use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Game {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub seller_id: Uuid,
    pub buyer_id: Uuid
}