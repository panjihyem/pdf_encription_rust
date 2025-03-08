use crate::domain::entities::PdfDocument;
use wasm_bindgen_test::*;
use rstest::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_new_pdf_document() {
    let name = "test.pdf";
    let content = vec![1, 2, 3, 4];
    let doc = PdfDocument::new(name.to_string(), content.clone());

    assert_eq!(doc.name, name);
    assert_eq!(doc.content, content);
    assert!(!doc.is_encrypted());
}

#[rstest]
#[case("test.pdf", vec![1, 2, 3, 4], "password123")]
#[case("sample.pdf", vec![5, 6, 7, 8], "secure_pass")]
#[wasm_bindgen_test]
fn test_pdf_encryption(#[case] name: &str, #[case] content: Vec<u8>, #[case] password: &str) {
    let mut doc = PdfDocument::new(name.to_string(), content.clone());
    
    // Test encryption
    doc.encrypt(password).expect("Failed to encrypt document");
    assert!(doc.is_encrypted());
    assert_ne!(doc.content, content);

    // Test decryption
    doc.decrypt(password).expect("Failed to decrypt document");
    assert!(!doc.is_encrypted());
    assert_eq!(doc.content, content);
}

#[wasm_bindgen_test]
fn test_invalid_decryption() {
    let mut doc = PdfDocument::new("test.pdf".to_string(), vec![1, 2, 3, 4]);
    
    doc.encrypt("correct_password").expect("Failed to encrypt document");
    let result = doc.decrypt("wrong_password");
    
    assert!(result.is_err());
    assert!(doc.is_encrypted());
}

#[wasm_bindgen_test]
fn test_pdf_metadata() {
    let content = vec![1; 1024]; // 1KB of data
    let doc = PdfDocument::new("test.pdf".to_string(), content);

    assert_eq!(doc.size, 1024);
    assert_eq!(doc.size_formatted(), "1.0 KB");
}

#[wasm_bindgen_test]
fn test_empty_pdf() {
    let doc = PdfDocument::new("empty.pdf".to_string(), vec![]);

    assert_eq!(doc.size, 0);
    assert_eq!(doc.size_formatted(), "0 B");
    assert!(!doc.is_encrypted());
}

#[wasm_bindgen_test]
fn test_large_pdf() {
    let content = vec![1; 5 * 1024 * 1024]; // 5MB of data
    let doc = PdfDocument::new("large.pdf".to_string(), content);

    assert_eq!(doc.size, 5 * 1024 * 1024);
    assert_eq!(doc.size_formatted(), "5.0 MB");
}

#[wasm_bindgen_test]
fn test_double_encryption() {
    let mut doc = PdfDocument::new("test.pdf".to_string(), vec![1, 2, 3, 4]);
    
    doc.encrypt("password123").expect("Failed to encrypt document");
    let result = doc.encrypt("another_password");
    
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn test_double_decryption() {
    let mut doc = PdfDocument::new("test.pdf".to_string(), vec![1, 2, 3, 4]);
    
    doc.encrypt("password123").expect("Failed to encrypt document");
    doc.decrypt("password123").expect("Failed to decrypt document");
    let result = doc.decrypt("password123");
    
    assert!(result.is_err());
}
