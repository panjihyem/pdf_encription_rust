use wasm_bindgen::prelude::*;
use web_sys::{File, Blob};
use js_sys::{Uint8Array, ArrayBuffer};
use std::io::Cursor;
use crate::domain::entities::pdf::PdfDocument;
use crate::application::ports::PdfProcessor;
use crate::infrastructure::error::{AppError, AppResult, pdf_error, encryption_error};
use crate::infrastructure::logging::{Logger, PerformanceMonitor};
use crate::infrastructure::validation::FileValidator;
use crate::infrastructure::encryption::EncryptionService;
use crate::infrastructure::integrity::IntegrityChecker;

#[wasm_bindgen]
pub struct WasmPdfProcessor {
    encryption_service: EncryptionService,
    integrity_checker: IntegrityChecker,
}

#[wasm_bindgen]
impl WasmPdfProcessor {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Logger::info("PdfProcessor", "Initializing PDF processor");
        Self {
            encryption_service: EncryptionService::new(),
            integrity_checker: IntegrityChecker::new(),
        }
    }

    pub async fn process_file(&self, file: File) -> AppResult<PdfDocument> {
        let monitor = PerformanceMonitor::start("process_file");
        Logger::info("PdfProcessor", &format!("Processing file: {}", file.name()));

        // Validate file before processing
        FileValidator::validate_pdf_file(&file)?;

        let array_buffer = wasm_bindgen_futures::JsFuture::from(file.array_buffer())
            .await
            .map_err(|e| {
                Logger::error("PdfProcessor", &format!("Failed to get array buffer: {:?}", e));
                pdf_error(format!("Failed to get array buffer: {:?}", e))
            })?;
        
        let uint8_array = Uint8Array::new(&array_buffer);
        let bytes = uint8_array.to_vec();

        // Verify PDF structure
        self.integrity_checker.verify_pdf_structure(&bytes)?;
        
        // Calculate initial hash for integrity tracking
        let initial_hash = IntegrityChecker::calculate_hash(&bytes);
        Logger::info("PdfProcessor", &format!("Initial file hash: {}", initial_hash));
        
        match pdf::file::FileOptions::cached()
            .load(&mut Cursor::new(bytes.clone())) {
            Ok(pdf_file) => {
                let metadata = pdf_file.trailer.info_dict.clone();
                let mut doc = PdfDocument::new(
                    file.name(),
                    file.size() as usize,
                    bytes,
                    metadata.map(|m| m.title.unwrap_or_default()),
                    Some(initial_hash),
                );

                // Store the hash for integrity verification
                doc.set_hash(initial_hash);
                
                Logger::info("PdfProcessor", "Successfully processed PDF file");
                monitor.end();
                Ok(doc)
            }
            Err(e) => {
                let error_msg = format!("Failed to process PDF: {}", e);
                Logger::error("PdfProcessor", &error_msg);
                monitor.end();
                Err(pdf_error(error_msg))
            }
        }
    }

    pub fn encrypt_content(&self, doc: &mut PdfDocument, password: &str) -> AppResult<()> {
        let monitor = PerformanceMonitor::start("encrypt_content");
        Logger::info("PdfProcessor", "Starting encryption process");

        // Validate password before encryption
        FileValidator::validate_password(password)?;
        
        // Verify content integrity before encryption
        let current_hash = IntegrityChecker::calculate_hash(&doc.content());
        if let Some(stored_hash) = doc.hash() {
            if current_hash != stored_hash {
                let error_msg = "Content integrity check failed before encryption";
                Logger::error("PdfProcessor", error_msg);
                return Err(encryption_error(error_msg));
            }
        }

        // Encrypt the PDF content
        let encrypted_content = self.encryption_service.encrypt(&doc.content(), password)?;
        
        // Update hash after encryption
        let new_hash = IntegrityChecker::calculate_hash(&encrypted_content);
        doc.set_content(encrypted_content);
        doc.set_hash(new_hash);
        doc.set_encrypted(true);
        
        Logger::info("PdfProcessor", "Successfully encrypted document");
        monitor.end();
        Ok(())
    }

    pub fn decrypt_content(&self, doc: &mut PdfDocument, password: &str) -> AppResult<()> {
        let monitor = PerformanceMonitor::start("decrypt_content");
        Logger::info("PdfProcessor", "Starting decryption process");

        // Validate password before decryption
        FileValidator::validate_password(password)?;
        
        // Verify content integrity before decryption
        let current_hash = IntegrityChecker::calculate_hash(&doc.content());
        if let Some(stored_hash) = doc.hash() {
            if current_hash != stored_hash {
                let error_msg = "Content integrity check failed before decryption";
                Logger::error("PdfProcessor", error_msg);
                return Err(encryption_error(error_msg));
            }
        }

        // Decrypt the PDF content
        let decrypted_content = self.encryption_service.decrypt(&doc.content(), password)?;
        
        // Verify PDF structure after decryption
        self.integrity_checker.verify_pdf_structure(&decrypted_content)?;
        
        // Update hash after decryption
        let new_hash = IntegrityChecker::calculate_hash(&decrypted_content);
        doc.set_content(decrypted_content);
        doc.set_hash(new_hash);
        doc.set_encrypted(false);
        
        Logger::info("PdfProcessor", "Successfully decrypted document");
        monitor.end();
        Ok(())
    }
}

impl PdfProcessor for WasmPdfProcessor {
    fn process(&self, file: File) -> Result<PdfDocument, String> {
        let future = self.process_file(file);
        wasm_bindgen_futures::spawn_local(async move {
            match future.await {
                Ok(doc) => Ok(doc),
                Err(e) => Err(e.to_string()),
            }
        });
        Ok(PdfDocument::default()) // Temporary until we implement proper async handling
    }

    fn encrypt(&self, doc: &mut PdfDocument, password: &str) -> Result<(), String> {
        self.encrypt_content(doc, password)
            .map_err(|e| e.to_string())
    }

    fn decrypt(&self, doc: &mut PdfDocument, password: &str) -> Result<(), String> {
        self.decrypt_content(doc, password)
            .map_err(|e| e.to_string())
    }
}
