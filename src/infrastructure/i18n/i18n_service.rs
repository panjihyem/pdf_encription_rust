use crate::domain::entities::Language;
use serde_json::Value;
use std::error::Error;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone)]
pub struct I18nService {
    current_language: Language,
    translations: std::collections::HashMap<Language, Value>,
}

impl I18nService {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut translations = std::collections::HashMap::new();
        
        // Load English translations
        let en_json = include_str!("en.json");
        let en_trans: Value = serde_json::from_str(en_json)?;
        translations.insert(Language::English, en_trans);

        // Load Indonesian translations
        let id_json = include_str!("id.json");
        let id_trans: Value = serde_json::from_str(id_json)?;
        translations.insert(Language::Indonesian, id_trans);

        Ok(Self {
            current_language: Language::English,
            translations,
        })
    }

    pub fn set_language(&mut self, language: Language) {
        self.current_language = language;
    }

    pub fn get_translation(&self, key: &str) -> Option<String> {
        let translations = self.translations.get(&self.current_language)?;
        let parts: Vec<&str> = key.split('.').collect();
        let mut current = translations;

        for part in parts {
            current = current.get(part)?;
        }

        current.as_str().map(|s| s.to_string())
    }

    pub fn get_current_language(&self) -> Language {
        self.current_language.clone()
    }

    pub fn format(&self, key: &str, params: &[(&str, &str)]) -> Option<String> {
        let mut text = self.get_translation(key)?;
        for (key, value) in params {
            text = text.replace(&format!("{{{}}}", key), value);
        }
        Some(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translation_loading() {
        let service = I18nService::new().unwrap();
        assert!(service.get_translation("app.title").is_some());
    }

    #[test]
    fn test_language_switching() {
        let mut service = I18nService::new().unwrap();
        
        // Test English
        assert_eq!(
            service.get_translation("actions.upload").unwrap(),
            "Upload PDF"
        );

        // Test Indonesian
        service.set_language(Language::Indonesian);
        assert_eq!(
            service.get_translation("actions.upload").unwrap(),
            "Unggah PDF"
        );
    }
}
