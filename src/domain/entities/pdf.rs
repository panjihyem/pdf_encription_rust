use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfDocument {
    pub id: String,
    pub name: String,
    pub content: Vec<u8>,
    pub encryption_status: EncryptionStatus,
    pub metadata: PdfMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfMetadata {
    pub size: usize,
    pub created_at: String,
    pub modified_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EncryptionStatus {
    Encrypted,
    Decrypted,
    Unknown,
}

impl PdfDocument {
    pub fn new(name: String, content: Vec<u8>) -> Self {
        let now = js_sys::Date::new_0().to_iso_string().as_string().unwrap();
        
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            encryption_status: EncryptionStatus::Unknown,
            metadata: PdfMetadata {
                size: content.len(),
                created_at: now.clone(),
                modified_at: now,
            },
            content,
        }
    }
}
