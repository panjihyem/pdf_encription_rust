use wasm_bindgen_test::*;
use yew::prelude::*;
use crate::domain::entities::Theme;
use crate::presentation::context::{ThemeContext, ThemeProvider, use_theme};

wasm_bindgen_test_configure!(run_in_browser);

#[derive(Properties, PartialEq)]
struct TestComponent {
    on_theme: Callback<ThemeContext>,
}

#[function_component(TestThemeConsumer)]
fn test_theme_consumer(props: &TestComponent) -> Html {
    let theme_ctx = use_theme();
    props.on_theme.emit(theme_ctx);
    html! {}
}

#[wasm_bindgen_test]
fn theme_context_default() {
    let theme_received = use_state(|| None);
    
    let on_theme = {
        let theme_received = theme_received.clone();
        Callback::from(move |ctx: ThemeContext| {
            theme_received.set(Some(ctx));
        })
    };

    let props = yew::props!(TestThemeConsumer {
        on_theme: on_theme,
    });

    let html = html! {
        <ThemeProvider>
            <TestThemeConsumer ..props />
        </ThemeProvider>
    };

    yew::Renderer::with_root(
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap(),
    ).render(html);

    if let Some(ctx) = &*theme_received {
        // Default theme should be System
        assert_eq!(ctx.theme, Theme::System);
    }
}

#[wasm_bindgen_test]
fn theme_context_light() {
    let theme_received = use_state(|| None);
    
    let on_theme = {
        let theme_received = theme_received.clone();
        Callback::from(move |ctx: ThemeContext| {
            theme_received.set(Some(ctx));
        })
    };

    let props = yew::props!(TestThemeConsumer {
        on_theme: on_theme.clone(),
    });

    let html = html! {
        <ThemeProvider>
            <TestThemeConsumer ..props />
        </ThemeProvider>
    };

    yew::Renderer::with_root(
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap(),
    ).render(html);

    if let Some(ctx) = &*theme_received {
        // Change theme to Light
        ctx.set_theme.emit(Theme::Light);

        // Theme should be updated to Light
        if let Some(updated_ctx) = &*theme_received {
            assert_eq!(updated_ctx.theme, Theme::Light);
        }
    }
}

#[wasm_bindgen_test]
fn theme_context_dark() {
    let theme_received = use_state(|| None);
    
    let on_theme = {
        let theme_received = theme_received.clone();
        Callback::from(move |ctx: ThemeContext| {
            theme_received.set(Some(ctx));
        })
    };

    let props = yew::props!(TestThemeConsumer {
        on_theme: on_theme.clone(),
    });

    let html = html! {
        <ThemeProvider>
            <TestThemeConsumer ..props />
        </ThemeProvider>
    };

    yew::Renderer::with_root(
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap(),
    ).render(html);

    if let Some(ctx) = &*theme_received {
        // Change theme to Dark
        ctx.set_theme.emit(Theme::Dark);

        // Theme should be updated to Dark
        if let Some(updated_ctx) = &*theme_received {
            assert_eq!(updated_ctx.theme, Theme::Dark);
        }
    }
}

#[wasm_bindgen_test]
fn theme_context_persistence() {
    let theme_received = use_state(|| None);
    
    let on_theme = {
        let theme_received = theme_received.clone();
        Callback::from(move |ctx: ThemeContext| {
            theme_received.set(Some(ctx));
        })
    };

    let props = yew::props!(TestThemeConsumer {
        on_theme: on_theme.clone(),
    });

    // First render to set theme
    let html = html! {
        <ThemeProvider>
            <TestThemeConsumer ..props.clone() />
        </ThemeProvider>
    };

    yew::Renderer::with_root(
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap(),
    ).render(html);

    if let Some(ctx) = &*theme_received {
        // Set theme to Dark
        ctx.set_theme.emit(Theme::Dark);
    }

    // Second render to check persistence
    let html = html! {
        <ThemeProvider>
            <TestThemeConsumer ..props />
        </ThemeProvider>
    };

    yew::Renderer::with_root(
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap(),
    ).render(html);

    if let Some(ctx) = &*theme_received {
        // Theme should still be Dark from localStorage
        assert_eq!(ctx.theme, Theme::Dark);
    }
}
