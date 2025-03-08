use crate::infrastructure::i18n::I18nService;
use crate::domain::entities::Language;
use wasm_bindgen_test::*;
use std::rc::Rc;

wasm_bindgen_test_configure!(run_in_browser);

fn create_i18n_service() -> Rc<I18nService> {
    let translations = vec![
        ("en", json!({
            "app": {
                "title": "PDF Encryption",
                "description": "Secure PDF encryption and decryption"
            },
            "upload": {
                "title": "Upload PDF",
                "drag_drop": "Drag and drop your PDF here",
                "or": "or",
                "browse": "Browse files",
                "supported_types": "Supported file type: PDF",
                "max_size": "Maximum file size: 10MB"
            },
            "preview": {
                "title": "PDF Preview",
                "no_document": "No document selected",
                "encrypted": "This document is encrypted",
                "page": "Page",
                "zoom_in": "Zoom in",
                "zoom_out": "Zoom out"
            },
            "encryption": {
                "title": "Encryption Controls",
                "password": {
                    "label": "Password",
                    "placeholder": "Enter password",
                    "show": "Show password",
                    "description": "Enter a password to encrypt/decrypt the PDF"
                },
                "actions": {
                    "encrypt": "Encrypt PDF",
                    "decrypt": "Decrypt PDF",
                    "clear": "Clear"
                }
            },
            "errors": {
                "invalid_password": "Invalid password",
                "encryption_failed": "Failed to encrypt document",
                "decryption_failed": "Failed to decrypt document",
                "invalid_file": "Invalid PDF file"
            }
        })),
        ("id", json!({
            "app": {
                "title": "Enkripsi PDF",
                "description": "Enkripsi dan dekripsi PDF yang aman"
            },
            "upload": {
                "title": "Unggah PDF",
                "drag_drop": "Tarik dan lepas PDF Anda di sini",
                "or": "atau",
                "browse": "Pilih berkas",
                "supported_types": "Tipe berkas yang didukung: PDF",
                "max_size": "Ukuran maksimum: 10MB"
            },
            "preview": {
                "title": "Pratinjau PDF",
                "no_document": "Tidak ada dokumen dipilih",
                "encrypted": "Dokumen ini terenkripsi",
                "page": "Halaman",
                "zoom_in": "Perbesar",
                "zoom_out": "Perkecil"
            },
            "encryption": {
                "title": "Kontrol Enkripsi",
                "password": {
                    "label": "Kata Sandi",
                    "placeholder": "Masukkan kata sandi",
                    "show": "Tampilkan kata sandi",
                    "description": "Masukkan kata sandi untuk mengenkripsi/mendekripsi PDF"
                },
                "actions": {
                    "encrypt": "Enkripsi PDF",
                    "decrypt": "Dekripsi PDF",
                    "clear": "Bersihkan"
                }
            },
            "errors": {
                "invalid_password": "Kata sandi tidak valid",
                "encryption_failed": "Gagal mengenkripsi dokumen",
                "decryption_failed": "Gagal mendekripsi dokumen",
                "invalid_file": "Berkas PDF tidak valid"
            }
        }))
    ];

    Rc::new(I18nService::new(translations))
}

#[wasm_bindgen_test]
fn test_i18n_initialization() {
    let i18n = create_i18n_service();
    assert_eq!(i18n.current_language(), Language::English);
}

#[wasm_bindgen_test]
fn test_english_translations() {
    let i18n = create_i18n_service();
    
    assert_eq!(
        i18n.get_translation("app.title").unwrap(),
        "PDF Encryption"
    );
    assert_eq!(
        i18n.get_translation("upload.drag_drop").unwrap(),
        "Drag and drop your PDF here"
    );
}

#[wasm_bindgen_test]
fn test_indonesian_translations() {
    let i18n = create_i18n_service();
    i18n.set_language(Language::Indonesian);
    
    assert_eq!(
        i18n.get_translation("app.title").unwrap(),
        "Enkripsi PDF"
    );
    assert_eq!(
        i18n.get_translation("upload.drag_drop").unwrap(),
        "Tarik dan lepas PDF Anda di sini"
    );
}

#[wasm_bindgen_test]
fn test_missing_translation() {
    let i18n = create_i18n_service();
    
    assert!(i18n.get_translation("nonexistent.key").is_none());
}

#[wasm_bindgen_test]
fn test_nested_translations() {
    let i18n = create_i18n_service();
    
    assert_eq!(
        i18n.get_translation("encryption.password.label").unwrap(),
        "Password"
    );
    assert_eq!(
        i18n.get_translation("encryption.actions.encrypt").unwrap(),
        "Encrypt PDF"
    );
}

#[wasm_bindgen_test]
fn test_language_switching() {
    let i18n = create_i18n_service();
    let key = "app.title";
    
    // Test English
    assert_eq!(i18n.get_translation(key).unwrap(), "PDF Encryption");
    
    // Switch to Indonesian
    i18n.set_language(Language::Indonesian);
    assert_eq!(i18n.get_translation(key).unwrap(), "Enkripsi PDF");
    
    // Switch back to English
    i18n.set_language(Language::English);
    assert_eq!(i18n.get_translation(key).unwrap(), "PDF Encryption");
}

#[wasm_bindgen_test]
fn test_error_messages() {
    let i18n = create_i18n_service();
    
    // Test English errors
    assert_eq!(
        i18n.get_translation("errors.invalid_password").unwrap(),
        "Invalid password"
    );
    
    // Test Indonesian errors
    i18n.set_language(Language::Indonesian);
    assert_eq!(
        i18n.get_translation("errors.invalid_password").unwrap(),
        "Kata sandi tidak valid"
    );
}

#[wasm_bindgen_test]
fn test_clone_and_equality() {
    let i18n1 = create_i18n_service();
    let i18n2 = i18n1.clone();
    
    assert_eq!(i18n1.current_language(), i18n2.current_language());
    
    // Change language in one instance
    i18n1.set_language(Language::Indonesian);
    assert_eq!(i18n1.current_language(), i18n2.current_language());
}
