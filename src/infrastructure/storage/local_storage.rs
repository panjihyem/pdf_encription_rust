use crate::application::ports::StoragePort;
use gloo_storage::{LocalStorage, Storage};
use std::error::Error;

pub struct LocalStorageImpl;

impl LocalStorageImpl {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait(?Send)]
impl StoragePort for LocalStorageImpl {
    async fn save(&self, key: &str, value: &[u8]) -> Result<(), Box<dyn Error>> {
        let encoded = base64::encode(value);
        LocalStorage::set(key, &encoded).map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    async fn load(&self, key: &str) -> Result<Option<Vec<u8>>, Box<dyn Error>> {
        match LocalStorage::get::<String>(key) {
            Ok(encoded) => {
                let decoded = base64::decode(&encoded)?;
                Ok(Some(decoded))
            }
            Err(_) => Ok(None),
        }
    }

    async fn delete(&self, key: &str) -> Result<(), Box<dyn Error>> {
        LocalStorage::delete(key);
        Ok(())
    }

    async fn clear(&self) -> Result<(), Box<dyn Error>> {
        LocalStorage::clear();
        Ok(())
    }
}
