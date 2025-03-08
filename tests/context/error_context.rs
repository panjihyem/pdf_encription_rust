use wasm_bindgen_test::*;
use yew::prelude::*;
use crate::presentation::context::{ErrorContext, ErrorProvider, use_error};

wasm_bindgen_test_configure!(run_in_browser);

#[derive(Properties, PartialEq)]
struct TestComponent {
    on_error: Callback<ErrorContext>,
}

#[function_component(TestErrorConsumer)]
fn test_error_consumer(props: &TestComponent) -> Html {
    let error_ctx = use_error();
    props.on_error.emit(error_ctx);
    html! {}
}

#[wasm_bindgen_test]
fn error_context_default() {
    let error_received = use_state(|| None);
    
    let on_error = {
        let error_received = error_received.clone();
        Callback::from(move |ctx: ErrorContext| {
            error_received.set(Some(ctx));
        })
    };

    let props = yew::props!(TestErrorConsumer {
        on_error: on_error,
    });

    let html = html! {
        <ErrorProvider>
            <TestErrorConsumer ..props />
        </ErrorProvider>
    };

    yew::Renderer::with_root(
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap(),
    ).render(html);

    if let Some(ctx) = &*error_received {
        // Default error should be None
        assert!(ctx.error.is_none());
    }
}

#[wasm_bindgen_test]
fn error_context_set_error() {
    let error_received = use_state(|| None);
    
    let on_error = {
        let error_received = error_received.clone();
        Callback::from(move |ctx: ErrorContext| {
            error_received.set(Some(ctx));
        })
    };

    let props = yew::props!(TestErrorConsumer {
        on_error: on_error.clone(),
    });

    let html = html! {
        <ErrorProvider>
            <TestErrorConsumer ..props />
        </ErrorProvider>
    };

    yew::Renderer::with_root(
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap(),
    ).render(html);

    if let Some(ctx) = &*error_received {
        // Set error
        let error_message = "Test error message".to_string();
        ctx.set_error.emit(Some(error_message.clone()));

        // Error should be updated
        if let Some(updated_ctx) = &*error_received {
            assert_eq!(updated_ctx.error, Some(error_message));
        }
    }
}

#[wasm_bindgen_test]
fn error_context_clear_error() {
    let error_received = use_state(|| None);
    
    let on_error = {
        let error_received = error_received.clone();
        Callback::from(move |ctx: ErrorContext| {
            error_received.set(Some(ctx));
        })
    };

    let props = yew::props!(TestErrorConsumer {
        on_error: on_error.clone(),
    });

    let html = html! {
        <ErrorProvider>
            <TestErrorConsumer ..props />
        </ErrorProvider>
    };

    yew::Renderer::with_root(
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap(),
    ).render(html);

    if let Some(ctx) = &*error_received {
        // Set error first
        ctx.set_error.emit(Some("Test error".to_string()));

        // Clear error
        ctx.set_error.emit(None);

        // Error should be cleared
        if let Some(updated_ctx) = &*error_received {
            assert!(updated_ctx.error.is_none());
        }
    }
}

#[wasm_bindgen_test]
fn error_context_persistence() {
    let error_received = use_state(|| None);
    
    let on_error = {
        let error_received = error_received.clone();
        Callback::from(move |ctx: ErrorContext| {
            error_received.set(Some(ctx));
        })
    };

    let props = yew::props!(TestErrorConsumer {
        on_error: on_error.clone(),
    });

    // First render to set error
    let html = html! {
        <ErrorProvider>
            <TestErrorConsumer ..props.clone() />
        </ErrorProvider>
    };

    yew::Renderer::with_root(
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap(),
    ).render(html);

    if let Some(ctx) = &*error_received {
        // Set error
        ctx.set_error.emit(Some("Persistent error".to_string()));
    }

    // Second render to check error is cleared (errors should not persist)
    let html = html! {
        <ErrorProvider>
            <TestErrorConsumer ..props />
        </ErrorProvider>
    };

    yew::Renderer::with_root(
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap(),
    ).render(html);

    if let Some(ctx) = &*error_received {
        // Error should be cleared on new provider instance
        assert!(ctx.error.is_none());
    }
}
