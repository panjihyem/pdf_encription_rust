use std::fmt;
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub enum AppError {
    PdfProcessing(String),
    Storage(String),
    Encryption(String),
    FileSystem(String),
    Network(String),
    Validation(String),
    Unknown(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::PdfProcessing(msg) => write!(f, "PDF Processing Error: {}", msg),
            AppError::Storage(msg) => write!(f, "Storage Error: {}", msg),
            AppError::Encryption(msg) => write!(f, "Encryption Error: {}", msg),
            AppError::FileSystem(msg) => write!(f, "File System Error: {}", msg),
            AppError::Network(msg) => write!(f, "Network Error: {}", msg),
            AppError::Validation(msg) => write!(f, "Validation Error: {}", msg),
            AppError::Unknown(msg) => write!(f, "Unknown Error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl From<JsValue> for AppError {
    fn from(value: JsValue) -> Self {
        AppError::Unknown(value.as_string().unwrap_or_default())
    }
}

impl From<AppError> for JsValue {
    fn from(error: AppError) -> Self {
        JsValue::from_str(&error.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;

// Helper functions for common error scenarios
pub fn pdf_error<T: ToString>(message: T) -> AppError {
    AppError::PdfProcessing(message.to_string())
}

pub fn storage_error<T: ToString>(message: T) -> AppError {
    AppError::Storage(message.to_string())
}

pub fn encryption_error<T: ToString>(message: T) -> AppError {
    AppError::Encryption(message.to_string())
}

pub fn validation_error<T: ToString>(message: T) -> AppError {
    AppError::Validation(message.to_string())
}

// Error handling utilities
pub fn log_error(error: &AppError) {
    web_sys::console::error_1(&error.to_string().into());
}

pub fn handle_error<T, E: std::error::Error>(result: Result<T, E>, context: &str) -> AppResult<T> {
    result.map_err(|e| AppError::Unknown(format!("{}: {}", context, e)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_error_display() {
        let error = AppError::PdfProcessing("Failed to load PDF".into());
        assert_eq!(
            error.to_string(),
            "PDF Processing Error: Failed to load PDF"
        );
    }

    #[wasm_bindgen_test]
    fn test_error_conversion() {
        let js_error = JsValue::from_str("Test error");
        let app_error: AppError = js_error.into();
        match app_error {
            AppError::Unknown(msg) => assert_eq!(msg, "Test error"),
            _ => panic!("Expected Unknown error variant"),
        }
    }

    #[wasm_bindgen_test]
    fn test_error_helpers() {
        let error = pdf_error("Test error");
        match error {
            AppError::PdfProcessing(msg) => assert_eq!(msg, "Test error"),
            _ => panic!("Expected PdfProcessing error variant"),
        }
    }
}
