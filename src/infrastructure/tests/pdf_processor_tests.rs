use wasm_bindgen_test::*;
use web_sys::{File, Blob};
use wasm_bindgen::JsCast;
use js_sys::{Uint8Array, Array};
use crate::infrastructure::pdf_processor::WasmPdfProcessor;
use crate::domain::entities::pdf::PdfDocument;

wasm_bindgen_test_configure!(run_in_browser);

fn create_test_file() -> File {
    let array = Uint8Array::new_with_length(10);
    let blob = Blob::new_with_u8_array_sequence(&Array::of1(&array)).unwrap();
    let file_options = web_sys::FilePropertyBag::new();
    file_options.type_("application/pdf");
    File::new_with_blob_sequence_and_options(
        &Array::of1(&blob.into()),
        "test.pdf",
        &file_options,
    ).unwrap()
}

#[wasm_bindgen_test]
async fn test_process_file() {
    let processor = WasmPdfProcessor::new();
    let file = create_test_file();
    
    let result = processor.process_file(file).await;
    assert!(result.is_ok(), "Should successfully process file");
    
    let doc = result.unwrap();
    assert_eq!(doc.name(), "test.pdf", "Document name should match");
    assert_eq!(doc.size(), 10, "Document size should match");
    assert!(!doc.is_encrypted(), "Document should not be encrypted initially");
}

#[wasm_bindgen_test]
async fn test_encryption() {
    let processor = WasmPdfProcessor::new();
    let file = create_test_file();
    
    let mut doc = processor.process_file(file).await.unwrap();
    let result = processor.encrypt_content(&mut doc, "password123");
    
    assert!(result.is_ok(), "Should successfully encrypt document");
    assert!(doc.is_encrypted(), "Document should be marked as encrypted");
}

#[wasm_bindgen_test]
async fn test_decryption() {
    let processor = WasmPdfProcessor::new();
    let file = create_test_file();
    
    let mut doc = processor.process_file(file).await.unwrap();
    processor.encrypt_content(&mut doc, "password123").unwrap();
    let result = processor.decrypt_content(&mut doc, "password123");
    
    assert!(result.is_ok(), "Should successfully decrypt document");
    assert!(!doc.is_encrypted(), "Document should be marked as not encrypted");
}
