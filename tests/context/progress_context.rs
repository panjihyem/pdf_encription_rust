use wasm_bindgen_test::*;
use yew::prelude::*;
use crate::presentation::context::{ProgressContext, ProgressProvider, use_progress};

wasm_bindgen_test_configure!(run_in_browser);

#[derive(Properties, PartialEq)]
struct TestComponent {
    on_progress: Callback<ProgressContext>,
}

#[function_component(TestProgressConsumer)]
fn test_progress_consumer(props: &TestComponent) -> Html {
    let progress_ctx = use_progress();
    props.on_progress.emit(progress_ctx);
    html! {}
}

#[wasm_bindgen_test]
fn progress_context_default() {
    let progress_received = use_state(|| None);
    
    let on_progress = {
        let progress_received = progress_received.clone();
        Callback::from(move |ctx: ProgressContext| {
            progress_received.set(Some(ctx));
        })
    };

    let props = yew::props!(TestProgressConsumer {
        on_progress: on_progress,
    });

    let html = html! {
        <ProgressProvider>
            <TestProgressConsumer ..props />
        </ProgressProvider>
    };

    yew::Renderer::with_root(
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap(),
    ).render(html);

    if let Some(ctx) = &*progress_received {
        // Default progress should be 0.0 with no message
        assert_eq!(ctx.progress, 0.0);
        assert!(ctx.message.is_none());
    }
}

#[wasm_bindgen_test]
fn progress_context_update_progress() {
    let progress_received = use_state(|| None);
    
    let on_progress = {
        let progress_received = progress_received.clone();
        Callback::from(move |ctx: ProgressContext| {
            progress_received.set(Some(ctx));
        })
    };

    let props = yew::props!(TestProgressConsumer {
        on_progress: on_progress.clone(),
    });

    let html = html! {
        <ProgressProvider>
            <TestProgressConsumer ..props />
        </ProgressProvider>
    };

    yew::Renderer::with_root(
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap(),
    ).render(html);

    if let Some(ctx) = &*progress_received {
        // Update progress with message
        ctx.set_progress.emit((50.0, Some("Halfway done".to_string())));

        // Progress should be updated
        if let Some(updated_ctx) = &*progress_received {
            assert_eq!(updated_ctx.progress, 50.0);
            assert_eq!(updated_ctx.message, Some("Halfway done".to_string()));
        }
    }
}

#[wasm_bindgen_test]
fn progress_context_complete() {
    let progress_received = use_state(|| None);
    
    let on_progress = {
        let progress_received = progress_received.clone();
        Callback::from(move |ctx: ProgressContext| {
            progress_received.set(Some(ctx));
        })
    };

    let props = yew::props!(TestProgressConsumer {
        on_progress: on_progress.clone(),
    });

    let html = html! {
        <ProgressProvider>
            <TestProgressConsumer ..props />
        </ProgressProvider>
    };

    yew::Renderer::with_root(
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap(),
    ).render(html);

    if let Some(ctx) = &*progress_received {
        // Set progress to 100%
        ctx.set_progress.emit((100.0, Some("Complete".to_string())));

        // Progress should be at 100%
        if let Some(updated_ctx) = &*progress_received {
            assert_eq!(updated_ctx.progress, 100.0);
            assert_eq!(updated_ctx.message, Some("Complete".to_string()));
        }
    }
}

#[wasm_bindgen_test]
fn progress_context_reset() {
    let progress_received = use_state(|| None);
    
    let on_progress = {
        let progress_received = progress_received.clone();
        Callback::from(move |ctx: ProgressContext| {
            progress_received.set(Some(ctx));
        })
    };

    let props = yew::props!(TestProgressConsumer {
        on_progress: on_progress.clone(),
    });

    let html = html! {
        <ProgressProvider>
            <TestProgressConsumer ..props />
        </ProgressProvider>
    };

    yew::Renderer::with_root(
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap(),
    ).render(html);

    if let Some(ctx) = &*progress_received {
        // Set some progress
        ctx.set_progress.emit((75.0, Some("Almost there".to_string())));

        // Reset progress
        ctx.set_progress.emit((0.0, None));

        // Progress should be reset
        if let Some(updated_ctx) = &*progress_received {
            assert_eq!(updated_ctx.progress, 0.0);
            assert!(updated_ctx.message.is_none());
        }
    }
}

#[wasm_bindgen_test]
fn progress_context_bounds() {
    let progress_received = use_state(|| None);
    
    let on_progress = {
        let progress_received = progress_received.clone();
        Callback::from(move |ctx: ProgressContext| {
            progress_received.set(Some(ctx));
        })
    };

    let props = yew::props!(TestProgressConsumer {
        on_progress: on_progress.clone(),
    });

    let html = html! {
        <ProgressProvider>
            <TestProgressConsumer ..props />
        </ProgressProvider>
    };

    yew::Renderer::with_root(
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap(),
    ).render(html);

    if let Some(ctx) = &*progress_received {
        // Test negative progress (should clamp to 0)
        ctx.set_progress.emit((-10.0, None));
        if let Some(updated_ctx) = &*progress_received {
            assert_eq!(updated_ctx.progress, 0.0);
        }

        // Test progress > 100 (should clamp to 100)
        ctx.set_progress.emit((150.0, None));
        if let Some(updated_ctx) = &*progress_received {
            assert_eq!(updated_ctx.progress, 100.0);
        }
    }
}

#[wasm_bindgen_test]
fn progress_context_message_only() {
    let progress_received = use_state(|| None);
    
    let on_progress = {
        let progress_received = progress_received.clone();
        Callback::from(move |ctx: ProgressContext| {
            progress_received.set(Some(ctx));
        })
    };

    let props = yew::props!(TestProgressConsumer {
        on_progress: on_progress.clone(),
    });

    let html = html! {
        <ProgressProvider>
            <TestProgressConsumer ..props />
        </ProgressProvider>
    };

    yew::Renderer::with_root(
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap(),
    ).render(html);

    if let Some(ctx) = &*progress_received {
        // Update only message, keep progress
        let current_progress = ctx.progress;
        ctx.set_progress.emit((current_progress, Some("New message".to_string())));

        // Progress should remain same with new message
        if let Some(updated_ctx) = &*progress_received {
            assert_eq!(updated_ctx.progress, current_progress);
            assert_eq!(updated_ctx.message, Some("New message".to_string()));
        }
    }
}
