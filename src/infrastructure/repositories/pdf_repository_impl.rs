use async_trait::async_trait;
use crate::domain::entities::PdfDocument;
use crate::domain::repositories::PdfRepository;
use std::error::Error;
use web_sys::Storage;

pub struct LocalPdfRepository {
    storage: Storage
}

impl LocalPdfRepository {
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }
}

#[async_trait(?Send)]
impl PdfRepository for LocalPdfRepository {
    async fn save(&self, pdf: &PdfDocument) -> Result<(), Box<dyn Error>> {
        let pdf_json = serde_json::to_string(pdf)?;
        self.storage.set_item(&pdf.id(), &pdf_json)?;
        Ok(())
    }

    async fn get(&self, id: &str) -> Result<Option<PdfDocument>, Box<dyn Error>> {
        match self.storage.get_item(id)? {
            Some(pdf_json) => {
                let pdf = serde_json::from_str(&pdf_json)?;
                Ok(Some(pdf))
            }
            None => Ok(None)
        }
    }

    async fn delete(&self, id: &str) -> Result<(), Box<dyn Error>> {
        self.storage.remove_item(id)?;
        Ok(())
    }

    async fn list(&self) -> Result<Vec<PdfDocument>, Box<dyn Error>> {
        let mut pdfs = Vec::new();
        let len = self.storage.length()?;

        for i in 0..len {
            if let Some(key) = self.storage.key(i)? {
                if let Some(pdf_json) = self.storage.get_item(&key)? {
                    let pdf: PdfDocument = serde_json::from_str(&pdf_json)?;
                    pdfs.push(pdf);
                }
            }
        }

        Ok(pdfs)
    }
}
