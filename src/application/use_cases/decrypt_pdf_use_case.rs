use wasm_bindgen::JsCast;
use web_sys::File;
use crate::domain::entities::PdfDocument;
use crate::domain::repositories::PdfRepository;
use crate::domain::error::{PdfError, validate_file_type, validate_password};
use crate::infrastructure::services::encryption::decrypt_pdf;

pub struct DecryptPdfUseCase {
    repository: Box<dyn PdfRepository>,
}

impl DecryptPdfUseCase {
    pub fn new() -> Self {
        Self {
            repository: Box::new(crate::infrastructure::repositories::LocalStoragePdfRepository::new()),
        }
    }

    pub async fn decrypt_pdf(&self, file: &File, password: &str) -> Result<PdfDocument, PdfError> {
        // Validate file type
        let file_type = file.type_();
        validate_file_type(&file_type)?;

        // Validate password
        validate_password(password)?;

        // Convert File to ArrayBuffer
        let array_buffer = file.array_buffer()
            .await
            .map_err(|e| PdfError::SystemError(e.to_string()))?;
        
        let uint8_array = js_sys::Uint8Array::new(&array_buffer);
        let bytes = uint8_array.to_vec();

        // Create PdfDocument from bytes
        let mut document = PdfDocument::new(file.name(), bytes)
            .map_err(|e| PdfError::InvalidFileFormat(e))?;

        // Verify document is encrypted
        if !document.is_encrypted() {
            return Err(PdfError::ValidationError("Document is not encrypted".to_string()));
        }

        // Attempt decryption
        let decrypted = decrypt_pdf(&document, password)
            .map_err(|e| PdfError::DecryptionFailed(e))?;

        // Save decrypted document
        self.repository.save(&decrypted)
            .map_err(|e| PdfError::StorageError(e))?;

        Ok(decrypted)
    }
}
