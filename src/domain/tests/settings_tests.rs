use crate::domain::entities::{Settings, Theme, Language};
use wasm_bindgen_test::*;
use rstest::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_default_settings() {
    let settings = Settings::default();
    
    assert_eq!(settings.theme, Theme::Light);
    assert_eq!(settings.language, Language::English);
}

#[rstest]
#[case(Theme::Dark, Language::Indonesian)]
#[case(Theme::Light, Language::English)]
#[wasm_bindgen_test]
fn test_settings_creation(#[case] theme: Theme, #[case] language: Language) {
    let settings = Settings::new(theme, language);
    
    assert_eq!(settings.theme, theme);
    assert_eq!(settings.language, language);
}

#[wasm_bindgen_test]
fn test_theme_toggle() {
    let mut settings = Settings::default();
    
    // Toggle from Light to Dark
    settings.toggle_theme();
    assert_eq!(settings.theme, Theme::Dark);
    
    // Toggle from Dark to Light
    settings.toggle_theme();
    assert_eq!(settings.theme, Theme::Light);
}

#[wasm_bindgen_test]
fn test_language_switch() {
    let mut settings = Settings::default();
    
    // Switch to Indonesian
    settings.set_language(Language::Indonesian);
    assert_eq!(settings.language, Language::Indonesian);
    
    // Switch back to English
    settings.set_language(Language::English);
    assert_eq!(settings.language, Language::English);
}

#[wasm_bindgen_test]
fn test_settings_serialization() {
    let settings = Settings::new(Theme::Dark, Language::Indonesian);
    
    let serialized = serde_json::to_string(&settings).expect("Failed to serialize settings");
    let deserialized: Settings = serde_json::from_str(&serialized).expect("Failed to deserialize settings");
    
    assert_eq!(settings.theme, deserialized.theme);
    assert_eq!(settings.language, deserialized.language);
}

#[wasm_bindgen_test]
fn test_theme_css_class() {
    assert_eq!(Theme::Light.css_class(), "theme-light");
    assert_eq!(Theme::Dark.css_class(), "theme-dark");
}

#[wasm_bindgen_test]
fn test_language_code() {
    assert_eq!(Language::English.code(), "en");
    assert_eq!(Language::Indonesian.code(), "id");
}

#[wasm_bindgen_test]
fn test_language_display() {
    assert_eq!(Language::English.to_string(), "English");
    assert_eq!(Language::Indonesian.to_string(), "Indonesian");
}

#[wasm_bindgen_test]
fn test_theme_display() {
    assert_eq!(Theme::Light.to_string(), "Light");
    assert_eq!(Theme::Dark.to_string(), "Dark");
}

#[wasm_bindgen_test]
fn test_settings_clone() {
    let settings = Settings::new(Theme::Dark, Language::Indonesian);
    let cloned = settings.clone();
    
    assert_eq!(settings.theme, cloned.theme);
    assert_eq!(settings.language, cloned.language);
}

#[wasm_bindgen_test]
fn test_settings_equality() {
    let settings1 = Settings::new(Theme::Dark, Language::Indonesian);
    let settings2 = Settings::new(Theme::Dark, Language::Indonesian);
    let settings3 = Settings::new(Theme::Light, Language::Indonesian);
    
    assert_eq!(settings1, settings2);
    assert_ne!(settings1, settings3);
}
