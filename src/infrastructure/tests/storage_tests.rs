use crate::infrastructure::storage::StorageService;
use crate::domain::entities::{Settings, Theme, Language};
use wasm_bindgen_test::*;
use gloo_storage::{LocalStorage, Storage};

wasm_bindgen_test_configure!(run_in_browser);

const SETTINGS_KEY: &str = "pdf_encrypt_settings";

fn clean_storage() {
    LocalStorage::delete(SETTINGS_KEY);
}

#[wasm_bindgen_test]
fn test_save_and_load_settings() {
    clean_storage();
    let storage = StorageService::new();
    let settings = Settings::new(Theme::Dark, Language::Indonesian);

    // Save settings
    storage.save_settings(&settings).expect("Failed to save settings");

    // Load settings
    let loaded = storage.load_settings().expect("Failed to load settings");

    assert_eq!(settings.theme, loaded.theme);
    assert_eq!(settings.language, loaded.language);
}

#[wasm_bindgen_test]
fn test_load_default_settings() {
    clean_storage();
    let storage = StorageService::new();

    let settings = storage.load_settings().expect("Failed to load default settings");

    assert_eq!(settings.theme, Theme::Light);
    assert_eq!(settings.language, Language::English);
}

#[wasm_bindgen_test]
fn test_update_settings() {
    clean_storage();
    let storage = StorageService::new();
    
    // Save initial settings
    let initial = Settings::new(Theme::Light, Language::English);
    storage.save_settings(&initial).expect("Failed to save initial settings");

    // Update settings
    let updated = Settings::new(Theme::Dark, Language::Indonesian);
    storage.save_settings(&updated).expect("Failed to save updated settings");

    // Load and verify
    let loaded = storage.load_settings().expect("Failed to load updated settings");
    assert_eq!(loaded.theme, Theme::Dark);
    assert_eq!(loaded.language, Language::Indonesian);
}

#[wasm_bindgen_test]
fn test_clear_settings() {
    clean_storage();
    let storage = StorageService::new();
    
    // Save settings
    let settings = Settings::new(Theme::Dark, Language::Indonesian);
    storage.save_settings(&settings).expect("Failed to save settings");

    // Clear storage
    LocalStorage::clear();

    // Load should return default settings
    let loaded = storage.load_settings().expect("Failed to load settings after clear");
    assert_eq!(loaded.theme, Theme::Light);
    assert_eq!(loaded.language, Language::English);
}

#[wasm_bindgen_test]
fn test_invalid_storage_data() {
    clean_storage();
    
    // Save invalid data
    LocalStorage::set(SETTINGS_KEY, "invalid json data").expect("Failed to set invalid data");

    let storage = StorageService::new();
    let settings = storage.load_settings().expect("Failed to handle invalid data");

    // Should return default settings
    assert_eq!(settings.theme, Theme::Light);
    assert_eq!(settings.language, Language::English);
}
