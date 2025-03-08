#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

pub mod components;
pub mod context;
pub mod integration;
pub mod state;
