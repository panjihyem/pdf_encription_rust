use crate::domain::entities::PdfDocument;
use std::error::Error;

#[async_trait(?Send)]
pub trait PdfEncryptionUseCase {
    async fn encrypt(&self, pdf: &mut PdfDocument, password: &str) -> Result<(), Box<dyn Error>>;
    async fn decrypt(&self, pdf: &mut PdfDocument, password: &str) -> Result<(), Box<dyn Error>>;
    async fn validate_password(&self, pdf: &PdfDocument, password: &str) -> Result<bool, Box<dyn Error>>;
}

pub struct PdfEncryptionService {
    crypto_port: Box<dyn CryptoPort>,
}

impl PdfEncryptionService {
    pub fn new(crypto_port: Box<dyn CryptoPort>) -> Self {
        Self { crypto_port }
    }
}

#[async_trait(?Send)]
impl PdfEncryptionUseCase for PdfEncryptionService {
    async fn encrypt(&self, pdf: &mut PdfDocument, password: &str) -> Result<(), Box<dyn Error>> {
        let encrypted_content = self.crypto_port.encrypt(&pdf.content, password).await?;
        pdf.content = encrypted_content;
        pdf.encryption_status = EncryptionStatus::Encrypted;
        Ok(())
    }

    async fn decrypt(&self, pdf: &mut PdfDocument, password: &str) -> Result<(), Box<dyn Error>> {
        let decrypted_content = self.crypto_port.decrypt(&pdf.content, password).await?;
        pdf.content = decrypted_content;
        pdf.encryption_status = EncryptionStatus::Decrypted;
        Ok(())
    }

    async fn validate_password(&self, pdf: &PdfDocument, password: &str) -> Result<bool, Box<dyn Error>> {
        self.crypto_port.validate_password(&pdf.content, password).await
    }
}

#[async_trait(?Send)]
pub trait CryptoPort {
    async fn encrypt(&self, data: &[u8], password: &str) -> Result<Vec<u8>, Box<dyn Error>>;
    async fn decrypt(&self, data: &[u8], password: &str) -> Result<Vec<u8>, Box<dyn Error>>;
    async fn validate_password(&self, data: &[u8], password: &str) -> Result<bool, Box<dyn Error>>;
}
