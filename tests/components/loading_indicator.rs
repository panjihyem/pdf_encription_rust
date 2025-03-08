use wasm_bindgen_test::*;
use yew::prelude::*;
use crate::presentation::components::{LoadingIndicator, LoadingIndicatorProps};
use super::test_utils::render_with_test_wrapper_and_props;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn loading_indicator_renders() {
    let props = LoadingIndicatorProps {
        message: Some("Loading...".to_string()),
        overlay: false,
        class: Classes::new(),
    };

    let html = render_with_test_wrapper_and_props::<LoadingIndicator>(props);
    
    // Assert component renders with message
    assert!(html.to_string().contains("Loading..."));
    
    // Assert spinner is present
    assert!(html.to_string().contains("animate-spin"));
}

#[wasm_bindgen_test]
fn loading_indicator_with_overlay() {
    let props = LoadingIndicatorProps {
        message: Some("Processing...".to_string()),
        overlay: true,
        class: Classes::new(),
    };

    let html = render_with_test_wrapper_and_props::<LoadingIndicator>(props);
    
    // Assert overlay is present
    assert!(html.to_string().contains("fixed inset-0"));
    assert!(html.to_string().contains("bg-black bg-opacity-50"));
    
    // Assert message is displayed
    assert!(html.to_string().contains("Processing..."));
}

#[wasm_bindgen_test]
fn loading_indicator_without_message() {
    let props = LoadingIndicatorProps {
        message: None,
        overlay: false,
        class: Classes::new(),
    };

    let html = render_with_test_wrapper_and_props::<LoadingIndicator>(props);
    
    // Assert spinner is present without message
    assert!(html.to_string().contains("animate-spin"));
    assert!(!html.to_string().contains("text-center"));
}
