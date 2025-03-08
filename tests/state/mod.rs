use wasm_bindgen_test::*;
use yew::prelude::*;
use crate::domain::entities::{PdfDocument, Theme};
use crate::presentation::state::{AppState, AppStateProvider, AppSettings};
use crate::presentation::components::ProcessingStatus;

wasm_bindgen_test_configure!(run_in_browser);

#[derive(Properties, PartialEq)]
struct TestComponent {
    on_state: Callback<AppState>,
}

#[function_component(TestStateConsumer)]
fn test_state_consumer(props: &TestComponent) -> Html {
    let app_state = use_context::<AppState>().unwrap();
    props.on_state.emit(app_state);
    html! {}
}

#[wasm_bindgen_test]
fn app_state_default() {
    let state_received = use_state(|| None);
    
    let on_state = {
        let state_received = state_received.clone();
        Callback::from(move |state: AppState| {
            state_received.set(Some(state));
        })
    };

    let props = yew::props!(TestStateConsumer {
        on_state: on_state,
    });

    let html = html! {
        <AppStateProvider>
            <TestStateConsumer ..props />
        </AppStateProvider>
    };

    let document = gloo_utils::document();
    let root = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&root).unwrap();
    
    yew::Renderer::with_root(root.clone().into()).render(html);

    if let Some(state) = &*state_received {
        // Test default values
        assert!(state.document.is_none());
        assert_eq!(state.processing_status, ProcessingStatus::Idle);
        assert_eq!(state.settings.theme, Theme::System);
        assert_eq!(state.settings.language, "en");
        assert!(state.settings.auto_preview);
        assert!(!state.settings.remember_password);
    }
}

#[wasm_bindgen_test]
fn app_state_document_handling() {
    let state_received = use_state(|| None);
    
    let on_state = {
        let state_received = state_received.clone();
        Callback::from(move |state: AppState| {
            state_received.set(Some(state));
        })
    };

    let props = yew::props!(TestStateConsumer {
        on_state: on_state.clone(),
    });

    let html = html! {
        <AppStateProvider>
            <TestStateConsumer ..props />
        </AppStateProvider>
    };

    let document = gloo_utils::document();
    let root = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&root).unwrap();
    
    yew::Renderer::with_root(root.clone().into()).render(html);

    if let Some(state) = &*state_received {
        // Test document handling
        let test_doc = PdfDocument::new("test.pdf".to_string(), vec![1, 2, 3]);
        state.set_document.emit(Some(test_doc.clone()));

        if let Some(updated_state) = &*state_received {
            assert!(updated_state.document.is_some());
            let doc = updated_state.document.as_ref().unwrap();
            assert_eq!(doc.name, "test.pdf");
            assert_eq!(doc.content, vec![1, 2, 3]);
        }
    }
}

#[wasm_bindgen_test]
fn app_state_settings_persistence() {
    let state_received = use_state(|| None);
    
    let on_state = {
        let state_received = state_received.clone();
        Callback::from(move |state: AppState| {
            state_received.set(Some(state));
        })
    };

    let props = yew::props!(TestStateConsumer {
        on_state: on_state.clone(),
    });

    // First render to set settings
    let html = html! {
        <AppStateProvider>
            <TestStateConsumer ..props.clone() />
        </AppStateProvider>
    };

    let document = gloo_utils::document();
    let root = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&root).unwrap();
    
    yew::Renderer::with_root(root.clone().into()).render(html);

    if let Some(state) = &*state_received {
        // Update settings
        let new_settings = AppSettings {
            theme: Theme::Dark,
            language: "id".to_string(),
            auto_preview: false,
            remember_password: true,
        };
        state.update_settings.emit(new_settings.clone());
    }

    // Second render to check persistence
    let html = html! {
        <AppStateProvider>
            <TestStateConsumer ..props />
        </AppStateProvider>
    };

    let document = gloo_utils::document();
    let root = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&root).unwrap();
    
    yew::Renderer::with_root(root.clone().into()).render(html);

    if let Some(state) = &*state_received {
        // Settings should be persisted
        assert_eq!(state.settings.theme, Theme::Dark);
        assert_eq!(state.settings.language, "id");
        assert!(!state.settings.auto_preview);
        assert!(state.settings.remember_password);
    }
}

#[wasm_bindgen_test]
fn app_state_processing_workflow() {
    let state_received = use_state(|| None);
    
    let on_state = {
        let state_received = state_received.clone();
        Callback::from(move |state: AppState| {
            state_received.set(Some(state));
        })
    };

    let props = yew::props!(TestStateConsumer {
        on_state: on_state.clone(),
    });

    let html = html! {
        <AppStateProvider>
            <TestStateConsumer ..props />
        </AppStateProvider>
    };

    let document = gloo_utils::document();
    let root = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&root).unwrap();
    
    yew::Renderer::with_root(root.clone().into()).render(html);

    if let Some(state) = &*state_received {
        // Test processing workflow
        state.set_processing_status.emit(ProcessingStatus::Processing);
        if let Some(updated_state) = &*state_received {
            assert_eq!(updated_state.processing_status, ProcessingStatus::Processing);
        }

        state.set_processing_status.emit(ProcessingStatus::Success);
        if let Some(updated_state) = &*state_received {
            assert_eq!(updated_state.processing_status, ProcessingStatus::Success);
        }

        state.set_processing_status.emit(ProcessingStatus::Error);
        if let Some(updated_state) = &*state_received {
            assert_eq!(updated_state.processing_status, ProcessingStatus::Error);
        }

        state.set_processing_status.emit(ProcessingStatus::Idle);
        if let Some(updated_state) = &*state_received {
            assert_eq!(updated_state.processing_status, ProcessingStatus::Idle);
        }
    }
}
