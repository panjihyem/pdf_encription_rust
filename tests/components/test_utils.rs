use yew::prelude::*;
use yew::virtual_dom::VNode;
use wasm_bindgen_test::*;
use crate::presentation::context::{
    ThemeProvider,
    LanguageProvider,
    ErrorProvider,
    LoadingProvider,
    ProgressProvider,
};

wasm_bindgen_test_configure!(run_in_browser);

pub fn render_with_test_wrapper<C>(children: Html) -> Html
where
    C: Component,
{
    html! {
        <ThemeProvider>
            <LanguageProvider>
                <ErrorProvider>
                    <LoadingProvider>
                        <ProgressProvider>
                            { children }
                        </ProgressProvider>
                    </LoadingProvider>
                </ErrorProvider>
            </LanguageProvider>
        </ThemeProvider>
    }
}

pub fn render_with_test_wrapper_and_props<C>(props: C::Properties) -> Html
where
    C: Component,
    C::Properties: PartialEq + Clone,
{
    render_with_test_wrapper::<C>(html! {
        <C ..props.clone() />
    })
}

pub fn render_to_node<C>(html: Html) -> web_sys::Node
where
    C: Component,
{
    let document = gloo_utils::document();
    let root = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&root).unwrap();
    
    yew::Renderer::<C>::with_root(root.clone().into()).render();
    
    root.into()
}

pub fn cleanup_test_node(node: web_sys::Node) {
    if let Some(parent) = node.parent_node() {
        parent.remove_child(&node).unwrap();
    }
}

pub fn simulate_click(element: &web_sys::Element) {
    let event = gloo_utils::document()
        .create_event("click")
        .unwrap();
    element
        .dispatch_event(&event)
        .unwrap();
}

pub fn simulate_input(element: &web_sys::Element, value: &str) {
    let event = gloo_utils::document()
        .create_event("input")
        .unwrap();
    element.set_text_content(Some(value));
    element
        .dispatch_event(&event)
        .unwrap();
}

pub fn find_element(root: &web_sys::Node, selector: &str) -> Option<web_sys::Element> {
    if let Some(element) = root.owner_document() {
        element.query_selector(selector).ok().flatten()
    } else {
        None
    }
}

pub fn find_elements(root: &web_sys::Node, selector: &str) -> Vec<web_sys::Element> {
    if let Some(element) = root.owner_document() {
        if let Ok(nodes) = element.query_selector_all(selector) {
            let mut elements = Vec::new();
            for i in 0..nodes.length() {
                if let Some(node) = nodes.get(i) {
                    elements.push(node);
                }
            }
            elements
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    }
}
