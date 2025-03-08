use yew::prelude::*;
use crate::infrastructure::i18n::I18nService;
use crate::domain::entities::Theme;
use crate::infrastructure::storage::StorageService;

pub struct Settings {
    i18n: I18nService,
    storage: StorageService,
    theme: Theme,
    language: String,
}

pub enum SettingsMsg {
    SetTheme(Theme),
    SetLanguage(String),
    SaveSettings,
    ResetSettings,
}

impl Component for Settings {
    type Message = SettingsMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let storage = StorageService::new().expect("Failed to initialize storage service");
        let theme = storage.get_theme().unwrap_or_default();
        let language = storage.get_language().unwrap_or_else(|| "en".to_string());
        
        Self {
            i18n: I18nService::new().expect("Failed to initialize i18n service"),
            storage,
            theme,
            language,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SettingsMsg::SetTheme(theme) => {
                self.theme = theme;
                true
            }
            SettingsMsg::SetLanguage(language) => {
                self.language = language;
                true
            }
            SettingsMsg::SaveSettings => {
                self.storage.set_theme(&self.theme);
                self.storage.set_language(&self.language);
                true
            }
            SettingsMsg::ResetSettings => {
                self.theme = Theme::default();
                self.language = "en".to_string();
                self.storage.set_theme(&self.theme);
                self.storage.set_language(&self.language);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        
        html! {
            <div class="container mx-auto px-4 py-8">
                <h1 class="text-3xl font-bold mb-8">
                    { self.i18n.t("settings") }
                </h1>

                <div class="max-w-2xl mx-auto bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6">
                    // Theme Settings
                    <div class="mb-8">
                        <h2 class="text-2xl font-semibold mb-4">
                            { self.i18n.t("theme_settings") }
                        </h2>
                        <div class="flex items-center space-x-4">
                            <button
                                onclick={link.callback(|_| SettingsMsg::SetTheme(Theme::Light))}
                                class={classes!(
                                    "px-4 py-2 rounded",
                                    if self.theme == Theme::Light { "bg-blue-500 text-white" } else { "bg-gray-200" }
                                )}
                            >
                                { self.i18n.t("light_theme") }
                            </button>
                            <button
                                onclick={link.callback(|_| SettingsMsg::SetTheme(Theme::Dark))}
                                class={classes!(
                                    "px-4 py-2 rounded",
                                    if self.theme == Theme::Dark { "bg-blue-500 text-white" } else { "bg-gray-200" }
                                )}
                            >
                                { self.i18n.t("dark_theme") }
                            </button>
                            <button
                                onclick={link.callback(|_| SettingsMsg::SetTheme(Theme::System))}
                                class={classes!(
                                    "px-4 py-2 rounded",
                                    if self.theme == Theme::System { "bg-blue-500 text-white" } else { "bg-gray-200" }
                                )}
                            >
                                { self.i18n.t("system_theme") }
                            </button>
                        </div>
                    </div>

                    // Language Settings
                    <div class="mb-8">
                        <h2 class="text-2xl font-semibold mb-4">
                            { self.i18n.t("language_settings") }
                        </h2>
                        <div class="flex items-center space-x-4">
                            <button
                                onclick={link.callback(|_| SettingsMsg::SetLanguage("en".to_string()))}
                                class={classes!(
                                    "px-4 py-2 rounded",
                                    if self.language == "en" { "bg-blue-500 text-white" } else { "bg-gray-200" }
                                )}
                            >
                                { "English" }
                            </button>
                            <button
                                onclick={link.callback(|_| SettingsMsg::SetLanguage("id".to_string()))}
                                class={classes!(
                                    "px-4 py-2 rounded",
                                    if self.language == "id" { "bg-blue-500 text-white" } else { "bg-gray-200" }
                                )}
                            >
                                { "Bahasa Indonesia" }
                            </button>
                        </div>
                    </div>

                    // Security Settings
                    <div class="mb-8">
                        <h2 class="text-2xl font-semibold mb-4">
                            { self.i18n.t("security_settings") }
                        </h2>
                        <div class="space-y-4">
                            <div class="flex items-center">
                                <span class="font-medium">{ self.i18n.t("encryption_algorithm") }:</span>
                                <span class="ml-2">{ "AES-256-GCM" }</span>
                            </div>
                            <div class="flex items-center">
                                <span class="font-medium">{ self.i18n.t("key_derivation") }:</span>
                                <span class="ml-2">{ "PBKDF2-SHA256" }</span>
                            </div>
                            <div class="flex items-center">
                                <span class="font-medium">{ self.i18n.t("iterations") }:</span>
                                <span class="ml-2">{ "100,000" }</span>
                            </div>
                        </div>
                    </div>

                    // Action Buttons
                    <div class="flex justify-end space-x-4">
                        <button
                            onclick={link.callback(|_| SettingsMsg::ResetSettings)}
                            class="px-4 py-2 bg-gray-500 text-white rounded hover:bg-gray-600"
                        >
                            { self.i18n.t("reset_settings") }
                        </button>
                        <button
                            onclick={link.callback(|_| SettingsMsg::SaveSettings)}
                            class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
                        >
                            { self.i18n.t("save_settings") }
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}
