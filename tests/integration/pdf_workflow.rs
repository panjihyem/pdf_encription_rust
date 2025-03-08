use wasm_bindgen_test::*;
use yew::prelude::*;
use std::sync::Arc;
use crate::domain::entities::PdfDocument;
use crate::application::use_cases::EncryptPdfUseCase;
use crate::infrastructure::repositories::LocalPdfRepository;
use crate::presentation::context::{LoadingProvider, use_loading_context};
use web_sys::window;

wasm_bindgen_test_configure!(run_in_browser);

const TEST_PASSWORD: &str = "StrongP@ssw0rd123";
const TEST_FILE_SIZE: usize = 1024 * 1024; // 1MB

fn setup_repository() -> Arc<dyn PdfRepository> {
    let window = window().unwrap();
    let local_storage = window.local_storage().unwrap().unwrap();
    Arc::new(LocalPdfRepository::new(local_storage))
}

#[wasm_bindgen_test]
async fn test_pdf_encryption_workflow() {
    let repository = setup_repository();
    let use_case = EncryptPdfUseCase::new(repository);
    
    // Setup test PDF document
    let test_pdf = PdfDocument::new_test(TEST_FILE_SIZE);
    
    // Validasi file
    assert!(test_pdf.validate_type(), "File type validation failed");
    assert!(test_pdf.validate_size(), "File size validation failed");
    
    // Test encryption menggunakan use case
    let encryption_result = use_case.execute(&test_pdf, TEST_PASSWORD).await;
    assert!(encryption_result.is_ok(), "Encryption failed");
    
    let encrypted_pdf = encryption_result.unwrap();
    assert!(encrypted_pdf.is_encrypted(), "PDF not marked as encrypted");
    
    // Verify encryption parameters
    let params = encrypted_pdf.get_encryption_params();
    assert_eq!(params.key_size, 256, "Incorrect key size");
    assert_eq!(params.iterations, 100_000, "Incorrect PBKDF2 iterations");
    assert_eq!(params.salt_size, 16, "Incorrect salt size");
    assert_eq!(params.nonce_size, 12, "Incorrect nonce size");
    
    // Verify PDF tersimpan di repository
    let stored_pdf = repository.get(&encrypted_pdf.id()).await.unwrap().unwrap();
    assert_eq!(stored_pdf.id(), encrypted_pdf.id(), "Stored PDF ID mismatch");
}

#[wasm_bindgen_test]
async fn test_invalid_password_validation() {
    let repository = setup_repository();
    let use_case = EncryptPdfUseCase::new(repository);
    let test_pdf = PdfDocument::new_test(TEST_FILE_SIZE);
    
    // Test dengan password yang terlalu pendek
    let weak_password = "Weak123!";
    let result = use_case.execute(&test_pdf, weak_password).await;
    assert!(result.is_err(), "Should reject weak password");
    
    // Test tanpa huruf besar
    let no_upper = "password123!@#";
    let result = use_case.execute(&test_pdf, no_upper).await;
    assert!(result.is_err(), "Should reject password without uppercase");
    
    // Test tanpa huruf kecil
    let no_lower = "PASSWORD123!@#";
    let result = use_case.execute(&test_pdf, no_lower).await;
    assert!(result.is_err(), "Should reject password without lowercase");
    
    // Test tanpa angka
    let no_digit = "Password!@#$%";
    let result = use_case.execute(&test_pdf, no_digit).await;
    assert!(result.is_err(), "Should reject password without digits");
    
    // Test tanpa simbol
    let no_symbol = "Password123456";
    let result = use_case.execute(&test_pdf, no_symbol).await;
    assert!(result.is_err(), "Should reject password without special characters");
}

#[wasm_bindgen_test]
async fn test_large_file_performance() {
    let repository = setup_repository();
    let use_case = EncryptPdfUseCase::new(repository);
    let large_pdf = PdfDocument::new_test(10 * 1024 * 1024); // 10MB
    
    // Measure encryption time
    let start = window()
        .unwrap()
        .performance()
        .unwrap()
        .now();
    
    let encryption_result = use_case.execute(&large_pdf, TEST_PASSWORD).await;
    
    let duration = window()
        .unwrap()
        .performance()
        .unwrap()
        .now() - start;
    
    assert!(encryption_result.is_ok(), "Large file encryption failed");
    assert!(duration < 5000.0, "Encryption took too long (>5s)");
    
    // Verify memory usage
    let memory = window()
        .unwrap()
        .performance()
        .unwrap()
        .memory()
        .unwrap();
    
    assert!(
        memory.used_js_heap_size() < 200 * 1024 * 1024, // 200MB
        "Memory usage too high"
    );
}

#[wasm_bindgen_test]
async fn test_concurrent_operations() {
    let repository = setup_repository();
    let use_case = EncryptPdfUseCase::new(Arc::clone(&repository));
    
    let test_pdfs: Vec<PdfDocument> = (0..5)
        .map(|_| PdfDocument::new_test(TEST_FILE_SIZE))
        .collect();
    
    let encryption_futures: Vec<_> = test_pdfs
        .iter()
        .map(|pdf| async {
            use_case.execute(pdf, TEST_PASSWORD).await
        })
        .collect();
    
    let results = futures::future::join_all(encryption_futures).await;
    
    // Verify all operations succeeded
    for result in results {
        assert!(result.is_ok(), "Concurrent encryption failed");
    }
    
    // Verify all PDFs tersimpan di repository
    let stored_pdfs = repository.list().await.unwrap();
    assert_eq!(stored_pdfs.len(), test_pdfs.len(), "Not all PDFs were stored");
}
