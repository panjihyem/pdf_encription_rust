use crate::domain::entities::UserSettings;
use crate::domain::repositories::SettingsRepository;
use std::error::Error;

#[async_trait(?Send)]
pub trait SettingsManagementUseCase {
    async fn update_settings(&self, settings: UserSettings) -> Result<(), Box<dyn Error>>;
    async fn get_settings(&self) -> Result<UserSettings, Box<dyn Error>>;
}

pub struct SettingsManagementService {
    settings_repository: Box<dyn SettingsRepository>,
}

impl SettingsManagementService {
    pub fn new(settings_repository: Box<dyn SettingsRepository>) -> Self {
        Self { settings_repository }
    }
}

#[async_trait(?Send)]
impl SettingsManagementUseCase for SettingsManagementService {
    async fn update_settings(&self, settings: UserSettings) -> Result<(), Box<dyn Error>> {
        self.settings_repository.save_settings(&settings).await
    }

    async fn get_settings(&self) -> Result<UserSettings, Box<dyn Error>> {
        self.settings_repository.get_settings().await
    }
}
