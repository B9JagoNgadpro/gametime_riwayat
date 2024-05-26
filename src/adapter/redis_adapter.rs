use async_trait::async_trait;
use redis::AsyncCommands;
use crate::adapter::cache::Cache;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct RedisAdapter {
    pub client: Arc<Mutex<redis::aio::MultiplexedConnection>>,
}

impl RedisAdapter {
    pub async fn new(redis_url: &str) -> Self {
        let client = redis::Client::open(redis_url).unwrap();
        let conn = client.get_multiplexed_async_connection().await.unwrap();
        RedisAdapter {
            client: Arc::new(Mutex::new(conn)),
        }
    }
}

#[async_trait]
impl Cache for RedisAdapter {
    async fn set(&self, key: &str, value: &str) {
        let mut conn = self.client.lock().await;
        let _: () = conn.set(key, value).await.unwrap();
    }

    async fn get(&self, key: &str) -> Option<String> {
        let mut conn = self.client.lock().await;
        conn.get(key).await.ok()
    }
}
