use wasm_bindgen_test::*;
use yew::prelude::*;
use std::time::Duration;
use crate::presentation::components::{Toast, ToastComponent, ToastContainer, ToastType};
use super::test_utils::render_with_test_wrapper_and_props;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn toast_creation() {
    let toast = Toast::new("Test message".to_string(), ToastType::Success);
    
    assert_eq!(toast.message, "Test message");
    assert_eq!(toast.toast_type, ToastType::Success);
    assert_eq!(toast.duration, Duration::from_secs(5)); // Default duration
}

#[wasm_bindgen_test]
fn toast_with_custom_duration() {
    let toast = Toast::new("Test message".to_string(), ToastType::Info)
        .with_duration(Duration::from_secs(10));
    
    assert_eq!(toast.duration, Duration::from_secs(10));
}

#[wasm_bindgen_test]
fn toast_component_success() {
    let toast = Toast::new("Success message".to_string(), ToastType::Success);
    let on_dismiss = Callback::from(|_| ());
    
    let props = yew::props!(ToastComponent {
        toast: toast,
        on_dismiss: on_dismiss,
    });

    let html = render_with_test_wrapper_and_props::<ToastComponent>(props);
    
    // Assert success styling
    assert!(html.to_string().contains("bg-green-500"));
    assert!(html.to_string().contains("Success message"));
}

#[wasm_bindgen_test]
fn toast_component_error() {
    let toast = Toast::new("Error message".to_string(), ToastType::Error);
    let on_dismiss = Callback::from(|_| ());
    
    let props = yew::props!(ToastComponent {
        toast: toast,
        on_dismiss: on_dismiss,
    });

    let html = render_with_test_wrapper_and_props::<ToastComponent>(props);
    
    // Assert error styling
    assert!(html.to_string().contains("bg-red-500"));
    assert!(html.to_string().contains("Error message"));
}

#[wasm_bindgen_test]
fn toast_component_warning() {
    let toast = Toast::new("Warning message".to_string(), ToastType::Warning);
    let on_dismiss = Callback::from(|_| ());
    
    let props = yew::props!(ToastComponent {
        toast: toast,
        on_dismiss: on_dismiss,
    });

    let html = render_with_test_wrapper_and_props::<ToastComponent>(props);
    
    // Assert warning styling
    assert!(html.to_string().contains("bg-yellow-500"));
    assert!(html.to_string().contains("Warning message"));
}

#[wasm_bindgen_test]
fn toast_component_info() {
    let toast = Toast::new("Info message".to_string(), ToastType::Info);
    let on_dismiss = Callback::from(|_| ());
    
    let props = yew::props!(ToastComponent {
        toast: toast,
        on_dismiss: on_dismiss,
    });

    let html = render_with_test_wrapper_and_props::<ToastComponent>(props);
    
    // Assert info styling
    assert!(html.to_string().contains("bg-blue-500"));
    assert!(html.to_string().contains("Info message"));
}

#[wasm_bindgen_test]
fn toast_container_multiple_toasts() {
    let props = yew::props!(ToastContainer {});
    let html = render_with_test_wrapper_and_props::<ToastContainer>(props);
    
    // Assert container structure
    assert!(html.to_string().contains("fixed bottom-4 right-4"));
    assert!(html.to_string().contains("flex flex-col gap-2"));
}

#[wasm_bindgen_test]
fn toast_dismiss_callback() {
    let toast = Toast::new("Test message".to_string(), ToastType::Success);
    let dismissed = use_state(|| false);
    
    let on_dismiss = {
        let dismissed = dismissed.clone();
        Callback::from(move |_| dismissed.set(true))
    };
    
    let props = yew::props!(ToastComponent {
        toast: toast,
        on_dismiss: on_dismiss,
    });

    let html = render_with_test_wrapper_and_props::<ToastComponent>(props);
    
    // Simulate clicking dismiss button
    let dismiss_btn = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector("button")
        .unwrap()
        .unwrap();
    
    dismiss_btn.click();
    
    // Assert callback was triggered
    assert!(*dismissed);
}
