use super::transaction::Transaction;

#[derive(Debug, Clone)]
pub struct HistoryBuilder {
    pub transactions: Vec<Transaction>,
}

impl HistoryBuilder {
    pub fn new() -> Self {
        Self {
            transactions: Vec::new(),
        }
    }

    pub async fn fetch_transactions(
        &mut self,
        transactions: Vec<Transaction>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.transactions.extend(transactions);
        Ok(())
    }

    pub fn build(self) -> History {
        History {
            transactions: self.transactions,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct History {
    pub transactions: Vec<Transaction>,
}