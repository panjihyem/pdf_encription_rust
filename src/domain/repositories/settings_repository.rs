use crate::domain::entities::UserSettings;
use std::error::Error;

#[async_trait(?Send)]
pub trait SettingsRepository {
    async fn save_settings(&self, settings: &UserSettings) -> Result<(), Box<dyn Error>>;
    async fn get_settings(&self) -> Result<UserSettings, Box<dyn Error>>;
}
