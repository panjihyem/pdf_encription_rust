use crate::domain::entities::PdfDocument;
use std::error::Error;

#[async_trait(?Send)]
pub trait PdfRepository {
    async fn save(&self, pdf: &PdfDocument) -> Result<(), Box<dyn Error>>;
    async fn get(&self, id: &str) -> Result<Option<PdfDocument>, Box<dyn Error>>;
    async fn delete(&self, id: &str) -> Result<(), Box<dyn Error>>;
    async fn list(&self) -> Result<Vec<PdfDocument>, Box<dyn Error>>;
}
