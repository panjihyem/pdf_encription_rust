use crate::application::ports::PdfServicePort;
use crate::domain::entities::PdfMetadata;
use pdf::file::FileOptions;
use std::error::Error;
use wasm_bindgen::prelude::*;

pub struct WasmPdfProcessor;

impl WasmPdfProcessor {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait(?Send)]
impl PdfServicePort for WasmPdfProcessor {
    async fn validate_pdf(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        match FileOptions::cached().load(data) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    async fn extract_metadata(&self, data: &[u8]) -> Result<PdfMetadata, Box<dyn Error>> {
        let doc = FileOptions::cached().load(data)?;
        let now = js_sys::Date::new_0().to_iso_string().as_string().unwrap();
        
        Ok(PdfMetadata {
            size: data.len(),
            created_at: now.clone(),
            modified_at: now,
        })
    }

    async fn create_preview(&self, pdf: &PdfDocument) -> Result<String, Box<dyn Error>> {
        // For now, we'll return a data URL for the first page
        // In a real implementation, we would render the PDF to canvas
        let preview_data = "data:application/pdf;base64,".to_string() + 
            &base64::encode(&pdf.content[..std::cmp::min(pdf.content.len(), 1024)]);
        Ok(preview_data)
    }
}
