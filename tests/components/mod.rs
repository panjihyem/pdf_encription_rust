use wasm_bindgen_test::*;
use yew::prelude::*;

mod test_utils;
pub use test_utils::*;

mod loading_indicator;
mod error_boundary;
mod navigation;
mod status_indicator;
mod toast;

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);
}
