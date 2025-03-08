use crate::domain::entities::PdfDocument;
use std::error::Error;

#[async_trait(?Send)]
pub trait PdfServicePort {
    async fn validate_pdf(&self, data: &[u8]) -> Result<bool, Box<dyn Error>>;
    async fn extract_metadata(&self, data: &[u8]) -> Result<PdfMetadata, Box<dyn Error>>;
    async fn create_preview(&self, pdf: &PdfDocument) -> Result<String, Box<dyn Error>>;
}
