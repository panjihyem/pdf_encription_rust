use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod application;
mod domain;
mod infrastructure;
mod presentation;

use presentation::app::App;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
    Ok(())
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
}
