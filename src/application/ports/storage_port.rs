use std::error::Error;

#[async_trait(?Send)]
pub trait StoragePort {
    async fn save(&self, key: &str, value: &[u8]) -> Result<(), Box<dyn Error>>;
    async fn load(&self, key: &str) -> Result<Option<Vec<u8>>, Box<dyn Error>>;
    async fn delete(&self, key: &str) -> Result<(), Box<dyn Error>>;
    async fn clear(&self) -> Result<(), Box<dyn Error>>;
}
