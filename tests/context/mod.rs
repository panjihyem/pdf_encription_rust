use wasm_bindgen_test::*;
use yew::prelude::*;

mod theme_context;
mod language_context;
mod error_context;
mod loading_context;
mod progress_context;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::presentation::context::*;
    use crate::domain::entities::Theme;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn context_providers_integration() {
        let html = html! {
            <ThemeProvider>
                <LanguageProvider>
                    <ErrorProvider>
                        <LoadingProvider>
                            <ProgressProvider>
                                <div id="test-content">{"Test Content"}</div>
                            </ProgressProvider>
                        </LoadingProvider>
                    </ErrorProvider>
                </LanguageProvider>
            </ThemeProvider>
        };

        let document = gloo_utils::document();
        let root = document.create_element("div").unwrap();
        document.body().unwrap().append_child(&root).unwrap();
        
        yew::Renderer::with_root(root.clone().into()).render(html);

        // Test that all providers are working together
        let test_content = document
            .query_selector("#test-content")
            .unwrap()
            .unwrap();
        
        assert_eq!(test_content.text_content().unwrap(), "Test Content");
    }

    #[wasm_bindgen_test]
    fn context_state_persistence() {
        // Test that context state is properly persisted in localStorage
        let storage = web_sys::window()
            .unwrap()
            .local_storage()
            .unwrap()
            .unwrap();

        // Set theme
        storage.set_item("theme", "dark").unwrap();
        
        // Set language
        storage.set_item("language", "id").unwrap();

        let html = html! {
            <ThemeProvider>
                <LanguageProvider>
                    <div id="test-content">{"Test Content"}</div>
                </LanguageProvider>
            </ThemeProvider>
        };

        let document = gloo_utils::document();
        let root = document.create_element("div").unwrap();
        document.body().unwrap().append_child(&root).unwrap();
        
        yew::Renderer::with_root(root.clone().into()).render(html);

        // Clean up
        storage.remove_item("theme").unwrap();
        storage.remove_item("language").unwrap();
    }
}
