use std::fmt;

#[derive(Debug)]
pub enum PdfError {
    // File related errors
    FileNotFound(String),
    InvalidFileFormat(String),
    FileTooLarge(usize),
    
    // Encryption related errors
    EncryptionFailed(String),
    DecryptionFailed(String),
    InvalidPassword(String),
    WeakPassword(String),
    
    // Storage related errors
    StorageError(String),
    
    // Validation errors
    ValidationError(String),
    
    // System errors
    SystemError(String),
    
    // Generic errors
    UnexpectedError(String),
}

impl fmt::Display for PdfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PdfError::FileNotFound(msg) => write!(f, "File not found: {}", msg),
            PdfError::InvalidFileFormat(msg) => write!(f, "Invalid file format: {}", msg),
            PdfError::FileTooLarge(size) => write!(f, "File too large: {} bytes", size),
            PdfError::EncryptionFailed(msg) => write!(f, "Encryption failed: {}", msg),
            PdfError::DecryptionFailed(msg) => write!(f, "Decryption failed: {}", msg),
            PdfError::InvalidPassword(msg) => write!(f, "Invalid password: {}", msg),
            PdfError::WeakPassword(msg) => write!(f, "Weak password: {}", msg),
            PdfError::StorageError(msg) => write!(f, "Storage error: {}", msg),
            PdfError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            PdfError::SystemError(msg) => write!(f, "System error: {}", msg),
            PdfError::UnexpectedError(msg) => write!(f, "Unexpected error: {}", msg),
        }
    }
}

impl std::error::Error for PdfError {}

impl From<web_sys::JsValue> for PdfError {
    fn from(value: web_sys::JsValue) -> Self {
        PdfError::UnexpectedError(value.as_string().unwrap_or_else(|| "Unknown JS error".to_string()))
    }
}

impl From<String> for PdfError {
    fn from(error: String) -> Self {
        PdfError::UnexpectedError(error)
    }
}

impl From<&str> for PdfError {
    fn from(error: &str) -> Self {
        PdfError::UnexpectedError(error.to_string())
    }
}

// Helper functions for common error checks
pub fn validate_file_size(size: usize, max_size: usize) -> Result<(), PdfError> {
    if size > max_size {
        Err(PdfError::FileTooLarge(size))
    } else {
        Ok(())
    }
}

pub fn validate_file_type(mime_type: &str) -> Result<(), PdfError> {
    if mime_type != "application/pdf" {
        Err(PdfError::InvalidFileFormat(format!(
            "Expected PDF file, got {}",
            mime_type
        )))
    } else {
        Ok(())
    }
}

pub fn validate_password(password: &str) -> Result<(), PdfError> {
    if password.len() < 8 {
        return Err(PdfError::WeakPassword(
            "Password must be at least 8 characters long".to_string(),
        ));
    }

    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_digit(10));
    let has_special = password.chars().any(|c| !c.is_alphanumeric());

    if !has_uppercase || !has_lowercase || !has_digit || !has_special {
        return Err(PdfError::WeakPassword(
            "Password must contain uppercase, lowercase, digit, and special characters".to_string(),
        ));
    }

    Ok(())
}
