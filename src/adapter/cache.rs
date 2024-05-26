use async_trait::async_trait;

#[async_trait]
pub trait Cache {
    async fn set(&self, key: &str, value: &str);
    async fn get(&self, key: &str) -> Option<String>;
}
