use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Language {
    #[serde(rename = "en")]
    English,
    #[serde(rename = "id")]
    Indonesian,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionPreferences {
    pub remember_password: bool,
    pub auto_clear_files: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettings {
    pub language: Language,
    pub theme: Theme,
    pub encryption_preferences: EncryptionPreferences,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            language: Language::English,
            theme: Theme::Light,
            encryption_preferences: EncryptionPreferences {
                remember_password: false,
                auto_clear_files: true,
            },
        }
    }
}
