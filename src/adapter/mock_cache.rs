use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use crate::adapter::cache::Cache;

pub struct MockCache {
    data: Arc<Mutex<HashMap<String, String>>>,
}

impl MockCache {
    pub fn new() -> Self {
        MockCache {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl Cache for MockCache {
    async fn set(&self, key: &str, value: &str) {
        let mut data = self.data.lock().await;
        data.insert(key.to_string(), value.to_string());
    }

    async fn get(&self, key: &str) -> Option<String> {
        let data = self.data.lock().await;
        data.get(key).cloned()
    }
}
