use yew::prelude::*;
use yew_router::prelude::*;
use crate::presentation::routing::Route;
use crate::presentation::context::{use_theme, use_language};
use crate::domain::entities::Theme;

#[function_component(NavigationMenu)]
pub fn navigation_menu() -> Html {
    let theme_ctx = use_theme();
    let lang_ctx = use_language();
    let navigator = use_navigator().unwrap();

    let toggle_theme = {
        let set_theme = theme_ctx.set_theme;
        Callback::from(move |_| {
            let new_theme = match theme_ctx.theme {
                Theme::Light => Theme::Dark,
                Theme::Dark => Theme::System,
                Theme::System => Theme::Light,
            };
            set_theme.emit(new_theme);
        })
    };

    let toggle_language = {
        let set_language = lang_ctx.set_language;
        Callback::from(move |_| {
            let new_lang = if lang_ctx.language == "en" {
                "id".to_string()
            } else {
                "en".to_string()
            };
            set_language.emit(new_lang);
        })
    };

    let theme_icon = match theme_ctx.theme {
        Theme::Light => "☀️",
        Theme::Dark => "🌙",
        Theme::System => "⚙️",
    };

    let lang_text = if lang_ctx.language == "en" { "EN" } else { "ID" };

    html! {
        <nav class="bg-white dark:bg-gray-800 shadow-lg">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex items-center justify-between h-16">
                    <div class="flex items-center">
                        <Link<Route> to={Route::Home} classes="text-gray-800 dark:text-white font-bold text-xl">
                            { "PDF Encrypt" }
                        </Link<Route>>
                    </div>

                    <div class="flex items-center space-x-4">
                        <Link<Route> to={Route::Home} classes="text-gray-600 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white px-3 py-2 rounded-md">
                            { lang_ctx.i18n.t("nav_home") }
                        </Link<Route>>
                        <Link<Route> to={Route::Encrypt} classes="text-gray-600 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white px-3 py-2 rounded-md">
                            { lang_ctx.i18n.t("nav_encrypt") }
                        </Link<Route>>
                        <Link<Route> to={Route::Decrypt} classes="text-gray-600 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white px-3 py-2 rounded-md">
                            { lang_ctx.i18n.t("nav_decrypt") }
                        </Link<Route>>
                        <Link<Route> to={Route::Settings} classes="text-gray-600 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white px-3 py-2 rounded-md">
                            { lang_ctx.i18n.t("nav_settings") }
                        </Link<Route>>

                        <button
                            onclick={toggle_theme}
                            class="p-2 rounded-md text-gray-600 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white focus:outline-none"
                            title={lang_ctx.i18n.t("toggle_theme")}
                        >
                            { theme_icon }
                        </button>

                        <button
                            onclick={toggle_language}
                            class="p-2 rounded-md text-gray-600 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white focus:outline-none"
                            title={lang_ctx.i18n.t("toggle_language")}
                        >
                            { lang_text }
                        </button>
                    </div>
                </div>
            </div>
        </nav>
    }
}
