use wasm_bindgen_test::*;
use yew::prelude::*;
use crate::presentation::components::{StatusIndicator, StatusIndicatorProps, ProcessingStatus};
use super::test_utils::render_with_test_wrapper_and_props;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn status_indicator_idle() {
    let props = StatusIndicatorProps {
        status: ProcessingStatus::Idle,
        message: "Ready".to_string(),
        icon: None,
        class: Classes::new(),
    };

    let html = render_with_test_wrapper_and_props::<StatusIndicator>(props);
    
    // Assert idle state styling
    assert!(html.to_string().contains("text-gray-500"));
    assert!(html.to_string().contains("Ready"));
    assert!(html.to_string().contains("⚪")); // Default idle icon
}

#[wasm_bindgen_test]
fn status_indicator_processing() {
    let props = StatusIndicatorProps {
        status: ProcessingStatus::Processing,
        message: "Encrypting PDF...".to_string(),
        icon: None,
        class: Classes::new(),
    };

    let html = render_with_test_wrapper_and_props::<StatusIndicator>(props);
    
    // Assert processing state styling
    assert!(html.to_string().contains("text-blue-500"));
    assert!(html.to_string().contains("Encrypting PDF..."));
    assert!(html.to_string().contains("🔄")); // Default processing icon
}

#[wasm_bindgen_test]
fn status_indicator_success() {
    let props = StatusIndicatorProps {
        status: ProcessingStatus::Success,
        message: "PDF encrypted successfully".to_string(),
        icon: None,
        class: Classes::new(),
    };

    let html = render_with_test_wrapper_and_props::<StatusIndicator>(props);
    
    // Assert success state styling
    assert!(html.to_string().contains("text-green-500"));
    assert!(html.to_string().contains("PDF encrypted successfully"));
    assert!(html.to_string().contains("✅")); // Default success icon
}

#[wasm_bindgen_test]
fn status_indicator_error() {
    let props = StatusIndicatorProps {
        status: ProcessingStatus::Error,
        message: "Encryption failed".to_string(),
        icon: None,
        class: Classes::new(),
    };

    let html = render_with_test_wrapper_and_props::<StatusIndicator>(props);
    
    // Assert error state styling
    assert!(html.to_string().contains("text-red-500"));
    assert!(html.to_string().contains("Encryption failed"));
    assert!(html.to_string().contains("❌")); // Default error icon
}

#[wasm_bindgen_test]
fn status_indicator_custom_icon() {
    let props = StatusIndicatorProps {
        status: ProcessingStatus::Success,
        message: "Custom icon test".to_string(),
        icon: Some("🔒".to_string()),
        class: Classes::new(),
    };

    let html = render_with_test_wrapper_and_props::<StatusIndicator>(props);
    
    // Assert custom icon is used
    assert!(html.to_string().contains("🔒"));
    assert!(!html.to_string().contains("✅")); // Default icon should not be present
}

#[wasm_bindgen_test]
fn status_indicator_custom_class() {
    let mut classes = Classes::new();
    classes.push("custom-class");
    
    let props = StatusIndicatorProps {
        status: ProcessingStatus::Idle,
        message: "Custom class test".to_string(),
        icon: None,
        class: classes,
    };

    let html = render_with_test_wrapper_and_props::<StatusIndicator>(props);
    
    // Assert custom class is applied
    assert!(html.to_string().contains("custom-class"));
}
