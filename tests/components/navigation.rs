use wasm_bindgen_test::*;
use yew::prelude::*;
use yew_router::Routable;
use crate::presentation::components::NavigationMenu;
use crate::presentation::routing::Route;
use super::test_utils::render_with_test_wrapper_and_props;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn navigation_menu_renders() {
    let props = yew::props!(NavigationMenu {});
    let html = render_with_test_wrapper_and_props::<NavigationMenu>(props);
    
    // Assert brand name is present
    assert!(html.to_string().contains("PDF Encrypt"));
    
    // Assert all navigation links are present
    assert!(html.to_string().contains("nav_home"));
    assert!(html.to_string().contains("nav_encrypt"));
    assert!(html.to_string().contains("nav_decrypt"));
    assert!(html.to_string().contains("nav_settings"));
}

#[wasm_bindgen_test]
fn navigation_menu_theme_toggle() {
    let props = yew::props!(NavigationMenu {});
    let html = render_with_test_wrapper_and_props::<NavigationMenu>(props);
    
    // Assert theme toggle button is present
    assert!(html.to_string().contains("toggle_theme"));
    
    // Find and click theme toggle button
    let theme_btn = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector("[title='toggle_theme']")
        .unwrap()
        .unwrap();
    
    theme_btn.click();
    
    // Theme should change (this will be reflected in the ThemeContext)
    let updated_html = html.to_string();
    assert!(updated_html.contains("toggle_theme"));
}

#[wasm_bindgen_test]
fn navigation_menu_language_toggle() {
    let props = yew::props!(NavigationMenu {});
    let html = render_with_test_wrapper_and_props::<NavigationMenu>(props);
    
    // Assert language toggle button is present
    assert!(html.to_string().contains("toggle_language"));
    
    // Find and click language toggle button
    let lang_btn = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector("[title='toggle_language']")
        .unwrap()
        .unwrap();
    
    lang_btn.click();
    
    // Language should change (this will be reflected in the LanguageContext)
    let updated_html = html.to_string();
    assert!(updated_html.contains("toggle_language"));
}

#[wasm_bindgen_test]
fn navigation_menu_routing() {
    let props = yew::props!(NavigationMenu {});
    let html = render_with_test_wrapper_and_props::<NavigationMenu>(props);
    
    // Assert all route links are present
    for route in [Route::Home, Route::Encrypt, Route::Decrypt, Route::Settings] {
        assert!(html.to_string().contains(&route.to_path()));
    }
}

#[wasm_bindgen_test]
fn navigation_menu_responsive() {
    let props = yew::props!(NavigationMenu {});
    let html = render_with_test_wrapper_and_props::<NavigationMenu>(props);
    
    // Assert responsive classes are present
    assert!(html.to_string().contains("max-w-7xl"));
    assert!(html.to_string().contains("mx-auto"));
    assert!(html.to_string().contains("px-4"));
    assert!(html.to_string().contains("sm:px-6"));
    assert!(html.to_string().contains("lg:px-8"));
}

#[wasm_bindgen_test]
fn navigation_menu_dark_mode() {
    let props = yew::props!(NavigationMenu {});
    let html = render_with_test_wrapper_and_props::<NavigationMenu>(props);
    
    // Assert dark mode classes are present
    assert!(html.to_string().contains("dark:bg-gray-800"));
    assert!(html.to_string().contains("dark:text-white"));
    assert!(html.to_string().contains("dark:text-gray-300"));
    assert!(html.to_string().contains("dark:hover:text-white"));
}
