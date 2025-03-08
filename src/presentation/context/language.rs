use yew::prelude::*;
use crate::infrastructure::i18n::I18nService;
use crate::infrastructure::storage::StorageService;

#[derive(Clone, Debug, PartialEq)]
pub struct LanguageContext {
    pub language: String,
    pub i18n: I18nService,
    pub set_language: Callback<String>,
}

#[derive(Properties, PartialEq)]
pub struct LanguageProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(LanguageProvider)]
pub fn language_provider(props: &LanguageProviderProps) -> Html {
    let storage = use_memo((), |_| {
        StorageService::new().expect("Failed to initialize storage service")
    });

    let language_state = use_state(|| storage.get_language().unwrap_or_else(|| "en".to_string()));
    let i18n = use_memo(*language_state, |lang| {
        I18nService::new_with_language(lang).expect("Failed to initialize i18n service")
    });

    let set_language = {
        let language_state = language_state.clone();
        let storage = storage.clone();
        Callback::from(move |new_language: String| {
            storage.set_language(&new_language);
            language_state.set(new_language);
        })
    };

    let context = LanguageContext {
        language: (*language_state).clone(),
        i18n: (*i18n).clone(),
        set_language,
    };

    html! {
        <ContextProvider<LanguageContext> context={context}>
            { for props.children.iter() }
        </ContextProvider<LanguageContext>>
    }
}

#[hook]
pub fn use_language() -> LanguageContext {
    use_context::<LanguageContext>().expect("LanguageContext not found")
}
