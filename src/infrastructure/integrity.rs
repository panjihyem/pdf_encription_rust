use ring::digest::{Context, SHA256};
use crate::infrastructure::error::{AppResult, validation_error};
use crate::infrastructure::logging::Logger;
use web_sys::File;
use wasm_bindgen::JsCast;
use js_sys::{Uint8Array, ArrayBuffer};
use wasm_bindgen_futures::JsFuture;

pub struct IntegrityChecker;

impl IntegrityChecker {
    pub fn new() -> Self {
        Logger::info("IntegrityChecker", "Initializing integrity checker");
        Self
    }

    pub fn calculate_hash(data: &[u8]) -> String {
        let mut context = Context::new(&SHA256);
        context.update(data);
        let digest = context.finish();
        hex::encode(digest.as_ref())
    }

    pub async fn verify_file_integrity(&self, file: &File, expected_hash: Option<&str>) -> AppResult<String> {
        Logger::info("IntegrityChecker", "Verifying file integrity");

        // Get file content as bytes
        let array_buffer = JsFuture::from(file.array_buffer())
            .await
            .map_err(|e| validation_error(format!("Failed to read file: {:?}", e)))?;

        let array_buffer: ArrayBuffer = array_buffer.dyn_into().unwrap();
        let uint8_array = Uint8Array::new(&array_buffer);
        let bytes = uint8_array.to_vec();

        // Calculate hash
        let hash = Self::calculate_hash(&bytes);

        // If expected hash is provided, verify it matches
        if let Some(expected) = expected_hash {
            if hash != expected {
                let error_msg = format!(
                    "File integrity check failed. Expected hash: {}, Got: {}",
                    expected, hash
                );
                Logger::error("IntegrityChecker", &error_msg);
                return Err(validation_error(error_msg));
            }
        }

        Logger::info("IntegrityChecker", "File integrity verified successfully");
        Ok(hash)
    }

    pub fn verify_pdf_structure(&self, data: &[u8]) -> AppResult<()> {
        Logger::info("IntegrityChecker", "Verifying PDF structure");

        // Check PDF magic number
        if data.len() < 5 || &data[0..5] != b"%PDF-" {
            let error_msg = "Invalid PDF format: Missing PDF signature";
            Logger::error("IntegrityChecker", error_msg);
            return Err(validation_error(error_msg));
        }

        // Check for EOF marker
        let eof_marker = b"%%EOF";
        let mut found_eof = false;
        for i in (0..data.len().saturating_sub(eof_marker.len())).rev() {
            if &data[i..i + eof_marker.len()] == eof_marker {
                found_eof = true;
                break;
            }
        }

        if !found_eof {
            let error_msg = "Invalid PDF format: Missing EOF marker";
            Logger::error("IntegrityChecker", error_msg);
            return Err(validation_error(error_msg));
        }

        // Basic structure validation using pdf crate
        match pdf::file::FileOptions::cached().load(&mut std::io::Cursor::new(data)) {
            Ok(_) => {
                Logger::info("IntegrityChecker", "PDF structure verified successfully");
                Ok(())
            }
            Err(e) => {
                let error_msg = format!("Invalid PDF structure: {}", e);
                Logger::error("IntegrityChecker", &error_msg);
                Err(validation_error(error_msg))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use js_sys::{Uint8Array, Array};
    use web_sys::{Blob, File, FilePropertyBag};

    wasm_bindgen_test_configure!(run_in_browser);

    fn create_test_pdf_file() -> File {
        let pdf_content = b"%PDF-1.7\nSome content\n%%EOF";
        let array = Uint8Array::new_with_length(pdf_content.len() as u32);
        array.copy_from(pdf_content);
        
        let blob = Blob::new_with_u8_array_sequence(&Array::of1(&array)).unwrap();
        let file_options = FilePropertyBag::new();
        file_options.type_("application/pdf");
        
        File::new_with_blob_sequence_and_options(
            &Array::of1(&blob.into()),
            "test.pdf",
            &file_options,
        ).unwrap()
    }

    #[wasm_bindgen_test]
    async fn test_hash_calculation() {
        let data = b"test data";
        let hash = IntegrityChecker::calculate_hash(data);
        assert!(!hash.is_empty());
    }

    #[wasm_bindgen_test]
    async fn test_file_integrity_verification() {
        let checker = IntegrityChecker::new();
        let file = create_test_pdf_file();
        
        let hash = checker.verify_file_integrity(&file, None).await.unwrap();
        assert!(!hash.is_empty());
        
        // Verify with correct hash
        let result = checker.verify_file_integrity(&file, Some(&hash)).await;
        assert!(result.is_ok());
        
        // Verify with incorrect hash
        let result = checker.verify_file_integrity(&file, Some("wrong_hash")).await;
        assert!(result.is_err());
    }

    #[wasm_bindgen_test]
    async fn test_pdf_structure_verification() {
        let checker = IntegrityChecker::new();
        
        // Valid PDF
        let valid_pdf = b"%PDF-1.7\nSome content\n%%EOF";
        assert!(checker.verify_pdf_structure(valid_pdf).is_ok());
        
        // Invalid PDF (no signature)
        let invalid_pdf = b"Not a PDF\n%%EOF";
        assert!(checker.verify_pdf_structure(invalid_pdf).is_err());
        
        // Invalid PDF (no EOF)
        let invalid_pdf = b"%PDF-1.7\nSome content";
        assert!(checker.verify_pdf_structure(invalid_pdf).is_err());
    }
}
