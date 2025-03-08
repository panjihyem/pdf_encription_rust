use crate::domain::entities::PdfDocument;
use crate::domain::repositories::PdfRepository;
use crate::infrastructure::services::encryption;
use std::sync::Arc;

pub struct EncryptPdfUseCase {
    pdf_repository: Arc<dyn PdfRepository>,
}

impl EncryptPdfUseCase {
    pub fn new(pdf_repository: Arc<dyn PdfRepository>) -> Self {
        Self { pdf_repository }
    }

    pub async fn execute(&self, pdf: &PdfDocument, password: &str) -> Result<PdfDocument, String> {
        // Validasi input
        if !Self::validate_password(password) {
            return Err("Password tidak memenuhi persyaratan keamanan".to_string());
        }

        if !pdf.validate_type() {
            return Err("File bukan PDF yang valid".to_string());
        }

        if !pdf.validate_size() {
            return Err("Ukuran file melebihi batas".to_string());
        }

        // Proses enkripsi
        let encrypted_pdf = encryption::encrypt_pdf(pdf, password)?;
        
        // Simpan PDF terenkripsi
        self.pdf_repository
            .save(&encrypted_pdf)
            .await
            .map_err(|e| e.to_string())?;

        Ok(encrypted_pdf)
    }

    fn validate_password(password: &str) -> bool {
        // Minimal 12 karakter
        if password.len() < 12 {
            return false;
        }

        // Harus mengandung huruf besar, huruf kecil, angka, dan simbol
        let has_upper = password.chars().any(|c| c.is_uppercase());
        let has_lower = password.chars().any(|c| c.is_lowercase());
        let has_digit = password.chars().any(|c| c.is_numeric());
        let has_special = password.chars().any(|c| !c.is_alphanumeric());

        has_upper && has_lower && has_digit && has_special
    }
}
