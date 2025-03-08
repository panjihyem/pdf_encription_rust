use wasm_bindgen_test::*;
use yew::prelude::*;
use crate::presentation::context::{LoadingProvider, use_loading_context};

wasm_bindgen_test_configure!(run_in_browser);

#[function_component(TestLoadingConsumer)]
fn test_loading_consumer() -> Html {
    let loading_ctx = use_loading_context();
    
    let onclick = {
        let set_loading = loading_ctx.set_loading.clone();
        Callback::from(move |_| {
            set_loading.emit((true, Some("Test loading...")));
        })
    };

    let onreset = {
        let set_loading = loading_ctx.set_loading.clone();
        Callback::from(move |_| {
            set_loading.emit((false, None));
        })
    };

    html! {
        <div>
            <button id="start-loading" onclick={onclick}>{"Start Loading"}</button>
            <button id="reset-loading" onclick={onreset}>{"Reset Loading"}</button>
            if loading_ctx.loading {
                <div id="loading-indicator">{"Loading..."}</div>
            }
            if let Some(message) = loading_ctx.message.as_ref() {
                <div id="loading-message">{message}</div>
            }
        </div>
    }
}

#[wasm_bindgen_test]
async fn test_loading_state_transitions() {
    let document = gloo_utils::document();
    let root = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&root).unwrap();

    let html = html! {
        <LoadingProvider>
            <TestLoadingConsumer />
        </LoadingProvider>
    };

    yew::Renderer::with_root(root.clone().into()).render(html);

    // Test initial state
    let loading_indicator = document.query_selector("#loading-indicator");
    assert!(loading_indicator.is_ok());
    assert!(loading_indicator.unwrap().is_none());

    // Test loading state transition
    let start_button = document.query_selector("#start-loading").unwrap().unwrap();
    start_button.click();

    // Wait for state update
    gloo_utils::next_frame().await;

    let loading_indicator = document.query_selector("#loading-indicator").unwrap().unwrap();
    let loading_message = document.query_selector("#loading-message").unwrap().unwrap();
    
    assert_eq!(loading_indicator.text_content().unwrap(), "Loading...");
    assert_eq!(loading_message.text_content().unwrap(), "Test loading...");

    // Test reset
    let reset_button = document.query_selector("#reset-loading").unwrap().unwrap();
    reset_button.click();

    // Wait for state update
    gloo_utils::next_frame().await;

    let loading_indicator = document.query_selector("#loading-indicator");
    assert!(loading_indicator.is_ok());
    assert!(loading_indicator.unwrap().is_none());

    let loading_message = document.query_selector("#loading-message");
    assert!(loading_message.is_ok());
    assert!(loading_message.unwrap().is_none());
}

#[wasm_bindgen_test]
async fn test_loading_message_updates() {
    let document = gloo_utils::document();
    let root = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&root).unwrap();

    let html = html! {
        <LoadingProvider>
            <TestLoadingConsumer />
        </LoadingProvider>
    };

    yew::Renderer::with_root(root.clone().into()).render(html);

    // Test message updates
    let loading_ctx = use_loading_context();
    loading_ctx.set_loading.emit((true, Some("Loading file...")));

    // Wait for state update
    gloo_utils::next_frame().await;

    let loading_message = document.query_selector("#loading-message").unwrap().unwrap();
    assert_eq!(loading_message.text_content().unwrap(), "Loading file...");

    loading_ctx.set_loading.emit((true, Some("Processing...")));

    // Wait for state update
    gloo_utils::next_frame().await;

    let loading_message = document.query_selector("#loading-message").unwrap().unwrap();
    assert_eq!(loading_message.text_content().unwrap(), "Processing...");
}
