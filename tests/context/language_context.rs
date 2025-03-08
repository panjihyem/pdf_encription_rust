use wasm_bindgen_test::*;
use yew::prelude::*;
use crate::presentation::context::{LanguageContext, LanguageProvider, use_language};

wasm_bindgen_test_configure!(run_in_browser);

#[derive(Properties, PartialEq)]
struct TestComponent {
    on_language: Callback<LanguageContext>,
}

#[function_component(TestLanguageConsumer)]
fn test_language_consumer(props: &TestComponent) -> Html {
    let lang_ctx = use_language();
    props.on_language.emit(lang_ctx);
    html! {}
}

#[wasm_bindgen_test]
fn language_context_default() {
    let lang_received = use_state(|| None);
    
    let on_language = {
        let lang_received = lang_received.clone();
        Callback::from(move |ctx: LanguageContext| {
            lang_received.set(Some(ctx));
        })
    };

    let props = yew::props!(TestLanguageConsumer {
        on_language: on_language,
    });

    let html = html! {
        <LanguageProvider>
            <TestLanguageConsumer ..props />
        </LanguageProvider>
    };

    yew::Renderer::with_root(
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap(),
    ).render(html);

    if let Some(ctx) = &*lang_received {
        // Default language should be English
        assert_eq!(ctx.language, "en");
        
        // Test basic translations
        assert!(ctx.i18n.t("nav_home").len() > 0);
        assert!(ctx.i18n.t("nav_encrypt").len() > 0);
        assert!(ctx.i18n.t("nav_decrypt").len() > 0);
    }
}

#[wasm_bindgen_test]
fn language_context_switch_to_indonesian() {
    let lang_received = use_state(|| None);
    
    let on_language = {
        let lang_received = lang_received.clone();
        Callback::from(move |ctx: LanguageContext| {
            lang_received.set(Some(ctx));
        })
    };

    let props = yew::props!(TestLanguageConsumer {
        on_language: on_language.clone(),
    });

    let html = html! {
        <LanguageProvider>
            <TestLanguageConsumer ..props />
        </LanguageProvider>
    };

    yew::Renderer::with_root(
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap(),
    ).render(html);

    if let Some(ctx) = &*lang_received {
        // Change language to Indonesian
        ctx.set_language.emit("id".to_string());

        // Language should be updated to Indonesian
        if let Some(updated_ctx) = &*lang_received {
            assert_eq!(updated_ctx.language, "id");
            
            // Test Indonesian translations
            let home_text = updated_ctx.i18n.t("nav_home");
            assert!(home_text.len() > 0);
            assert_ne!(home_text, "Home"); // Should not be English
        }
    }
}

#[wasm_bindgen_test]
fn language_context_persistence() {
    let lang_received = use_state(|| None);
    
    let on_language = {
        let lang_received = lang_received.clone();
        Callback::from(move |ctx: LanguageContext| {
            lang_received.set(Some(ctx));
        })
    };

    let props = yew::props!(TestLanguageConsumer {
        on_language: on_language.clone(),
    });

    // First render to set language
    let html = html! {
        <LanguageProvider>
            <TestLanguageConsumer ..props.clone() />
        </LanguageProvider>
    };

    yew::Renderer::with_root(
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap(),
    ).render(html);

    if let Some(ctx) = &*lang_received {
        // Set language to Indonesian
        ctx.set_language.emit("id".to_string());
    }

    // Second render to check persistence
    let html = html! {
        <LanguageProvider>
            <TestLanguageConsumer ..props />
        </LanguageProvider>
    };

    yew::Renderer::with_root(
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap(),
    ).render(html);

    if let Some(ctx) = &*lang_received {
        // Language should still be Indonesian from localStorage
        assert_eq!(ctx.language, "id");
    }
}

#[wasm_bindgen_test]
fn language_context_translation_keys() {
    let lang_received = use_state(|| None);
    
    let on_language = {
        let lang_received = lang_received.clone();
        Callback::from(move |ctx: LanguageContext| {
            lang_received.set(Some(ctx));
        })
    };

    let props = yew::props!(TestLanguageConsumer {
        on_language: on_language,
    });

    let html = html! {
        <LanguageProvider>
            <TestLanguageConsumer ..props />
        </LanguageProvider>
    };

    yew::Renderer::with_root(
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap(),
    ).render(html);

    if let Some(ctx) = &*lang_received {
        // Test all required translation keys are present
        let required_keys = [
            "nav_home",
            "nav_encrypt",
            "nav_decrypt",
            "nav_settings",
            "toggle_theme",
            "toggle_language",
            "error_occurred",
            "try_again",
            "loading",
            "processing",
            "success",
            "error",
        ];

        for key in required_keys.iter() {
            let translation = ctx.i18n.t(key);
            assert!(translation.len() > 0, "Missing translation for key: {}", key);
        }
    }
}
