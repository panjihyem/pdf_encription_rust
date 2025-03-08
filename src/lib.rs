use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod application;
mod domain;
mod infrastructure;
mod presentation;

use presentation::app::App;

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    // Initialize logging
    wasm_logger::init(wasm_logger::Config::default());
    console_log::init_with_level(log::Level::Debug).expect("Failed to initialize logger");
    
    // Initialize storage
    gloo::storage::LocalStorage::raw();
    
    // Start the Yew application
    yew::Renderer::<App>::new().render();
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    fn test_initialization() {
        assert!(run_app().is_ok());
    }
}
