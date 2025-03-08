use std::collections::HashMap;
use ring::digest::{Context, SHA256};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfDocument {
    id: String,
    content: Vec<u8>,
    encrypted: bool,
    encryption_params: Option<EncryptionParams>,
    size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfMetadata {
    pub size: usize,
    pub name: String,
    pub content_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionParams {
    pub key_size: usize,
    pub iterations: usize,
    pub salt_size: usize,
    pub nonce_size: usize,
}

impl PdfDocument {
    pub fn new(content: Vec<u8>, name: String) -> Self {
        let size = content.len();
        let metadata = PdfMetadata {
            size,
            name,
            content_type: "application/pdf".to_string(),
        };

        Self {
            id: Uuid::new_v4().to_string(),
            content,
            encrypted: false,
            encryption_params: None,
            size,
        }
    }

    #[cfg(test)]
    pub fn new_test(size: usize) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            content: vec![0; size],
            encrypted: false,
            encryption_params: None,
            size,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn content(&self) -> &[u8] {
        &self.content
    }

    pub fn is_encrypted(&self) -> bool {
        self.encrypted
    }

    pub fn get_encryption_params(&self) -> &EncryptionParams {
        self.encryption_params.as_ref().expect("Document not encrypted")
    }

    pub fn set_encrypted(&mut self, encrypted_content: Vec<u8>, params: EncryptionParams) {
        self.content = encrypted_content;
        self.encrypted = true;
        self.encryption_params = Some(params);
    }

    pub fn validate_type(&self) -> bool {
        // Validasi header PDF
        if self.content.len() < 5 {
            return false;
        }
        
        let header = &self.content[0..5];
        header == b"%PDF-"
    }

    pub fn validate_size(&self) -> bool {
        // Maksimal ukuran file 100MB
        const MAX_SIZE: usize = 100 * 1024 * 1024;
        self.size <= MAX_SIZE
    }

    pub fn get_content_hash(&self) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(&self.content);
        format!("{:x}", hasher.finalize())
    }
}
