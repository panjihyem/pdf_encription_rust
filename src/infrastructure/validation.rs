use web_sys::File;
use crate::infrastructure::error::{AppResult, validation_error};
use crate::infrastructure::logging::Logger;

const MAX_FILE_SIZE: usize = 100 * 1024 * 1024; // 100MB
const ALLOWED_MIME_TYPES: [&str; 1] = ["application/pdf"];

pub struct FileValidator;

impl FileValidator {
    pub fn validate_pdf_file(file: &File) -> AppResult<()> {
        Logger::info("FileValidator", &format!("Validating file: {}", file.name()));
        
        // Check file size
        let file_size = file.size() as usize;
        if file_size > MAX_FILE_SIZE {
            let error_msg = format!(
                "File size {} bytes exceeds maximum allowed size of {} bytes",
                file_size,
                MAX_FILE_SIZE
            );
            Logger::error("FileValidator", &error_msg);
            return Err(validation_error(error_msg));
        }

        // Check MIME type
        let file_type = file.type_();
        if !ALLOWED_MIME_TYPES.contains(&file_type.as_str()) {
            let error_msg = format!(
                "Invalid file type: {}. Expected one of: {:?}",
                file_type,
                ALLOWED_MIME_TYPES
            );
            Logger::error("FileValidator", &error_msg);
            return Err(validation_error(error_msg));
        }

        Logger::info("FileValidator", "File validation successful");
        Ok(())
    }

    pub fn validate_password(password: &str) -> AppResult<()> {
        Logger::info("FileValidator", "Validating password");

        if password.is_empty() {
            let error_msg = "Password cannot be empty";
            Logger::error("FileValidator", error_msg);
            return Err(validation_error(error_msg));
        }

        if password.len() < 8 {
            let error_msg = "Password must be at least 8 characters long";
            Logger::error("FileValidator", error_msg);
            return Err(validation_error(error_msg));
        }

        // Check for at least one number
        if !password.chars().any(|c| c.is_numeric()) {
            let error_msg = "Password must contain at least one number";
            Logger::error("FileValidator", error_msg);
            return Err(validation_error(error_msg));
        }

        // Check for at least one special character
        let special_chars = "!@#$%^&*()_+-=[]{}|;:,.<>?";
        if !password.chars().any(|c| special_chars.contains(c)) {
            let error_msg = "Password must contain at least one special character";
            Logger::error("FileValidator", error_msg);
            return Err(validation_error(error_msg));
        }

        Logger::info("FileValidator", "Password validation successful");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use js_sys::{Uint8Array, Array};
    use web_sys::{Blob, File};

    wasm_bindgen_test_configure!(run_in_browser);

    fn create_test_file(size: u32, mime_type: &str) -> File {
        let array = Uint8Array::new_with_length(size);
        let blob = Blob::new_with_u8_array_sequence(&Array::of1(&array)).unwrap();
        let file_options = web_sys::FilePropertyBag::new();
        file_options.type_(mime_type);
        File::new_with_blob_sequence_and_options(
            &Array::of1(&blob.into()),
            "test.pdf",
            &file_options,
        ).unwrap()
    }

    #[wasm_bindgen_test]
    fn test_valid_pdf_file() {
        let file = create_test_file(1024, "application/pdf");
        let result = FileValidator::validate_pdf_file(&file);
        assert!(result.is_ok(), "Should accept valid PDF file");
    }

    #[wasm_bindgen_test]
    fn test_invalid_file_type() {
        let file = create_test_file(1024, "text/plain");
        let result = FileValidator::validate_pdf_file(&file);
        assert!(result.is_err(), "Should reject non-PDF file");
    }

    #[wasm_bindgen_test]
    fn test_file_too_large() {
        let file = create_test_file(MAX_FILE_SIZE as u32 + 1, "application/pdf");
        let result = FileValidator::validate_pdf_file(&file);
        assert!(result.is_err(), "Should reject file larger than maximum size");
    }

    #[wasm_bindgen_test]
    fn test_valid_password() {
        let result = FileValidator::validate_password("Password123!");
        assert!(result.is_ok(), "Should accept valid password");
    }

    #[wasm_bindgen_test]
    fn test_invalid_password() {
        let result = FileValidator::validate_password("");
        assert!(result.is_err(), "Should reject empty password");

        let result = FileValidator::validate_password("short");
        assert!(result.is_err(), "Should reject short password");

        let result = FileValidator::validate_password("password");
        assert!(result.is_err(), "Should reject password without number");

        let result = FileValidator::validate_password("password123");
        assert!(result.is_err(), "Should reject password without special character");
    }
}
