use wasm_bindgen_test::*;
use yew::prelude::*;
use crate::presentation::components::{ErrorBoundary, ErrorBoundaryProps};
use super::test_utils::render_with_test_wrapper_and_props;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn error_boundary_renders_children() {
    let props = ErrorBoundaryProps {
        children: html! {
            <div id="test-child">{"Test Content"}</div>
        },
        fallback: None,
    };

    let html = render_with_test_wrapper_and_props::<ErrorBoundary>(props);
    
    // Assert children are rendered
    assert!(html.to_string().contains("test-child"));
    assert!(html.to_string().contains("Test Content"));
}

#[wasm_bindgen_test]
fn error_boundary_shows_default_error() {
    let props = ErrorBoundaryProps {
        children: html! {
            <div>
                {
                    let _err = std::panic::panic_any("Test error");
                }
            </div>
        },
        fallback: None,
    };

    let html = render_with_test_wrapper_and_props::<ErrorBoundary>(props);
    
    // Assert error message is displayed
    assert!(html.to_string().contains("error_occurred"));
    assert!(html.to_string().contains("Test error"));
    assert!(html.to_string().contains("try_again"));
}

#[wasm_bindgen_test]
fn error_boundary_shows_custom_fallback() {
    let props = ErrorBoundaryProps {
        children: html! {
            <div>
                {
                    let _err = std::panic::panic_any("Test error");
                }
            </div>
        },
        fallback: Some(html! {
            <div id="custom-error">{"Custom Error"}</div>
        }),
    };

    let html = render_with_test_wrapper_and_props::<ErrorBoundary>(props);
    
    // Assert custom fallback is displayed
    assert!(html.to_string().contains("custom-error"));
    assert!(html.to_string().contains("Custom Error"));
}

#[wasm_bindgen_test]
fn error_boundary_clears_error() {
    let props = ErrorBoundaryProps {
        children: html! {
            <div>
                {
                    let _err = std::panic::panic_any("Test error");
                }
            </div>
        },
        fallback: None,
    };

    let html = render_with_test_wrapper_and_props::<ErrorBoundary>(props);
    
    // Assert error is displayed
    assert!(html.to_string().contains("error_occurred"));
    
    // Simulate clicking try again button
    let try_again_btn = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector("button")
        .unwrap()
        .unwrap();
    
    try_again_btn.click();
    
    // Assert error is cleared
    let updated_html = html.to_string();
    assert!(!updated_html.contains("error_occurred"));
    assert!(!updated_html.contains("Test error"));
}
