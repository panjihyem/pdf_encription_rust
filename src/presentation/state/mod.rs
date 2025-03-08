use yew::prelude::*;
use std::rc::Rc;
use crate::domain::entities::{PdfDocument, Theme};
use crate::presentation::components::ProcessingStatus;

#[derive(Clone, Debug, PartialEq)]
pub struct AppSettings {
    pub theme: Theme,
    pub language: String,
    pub auto_preview: bool,
    pub remember_password: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: Theme::System,
            language: "en".to_string(),
            auto_preview: true,
            remember_password: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AppState {
    pub document: Option<PdfDocument>,
    pub processing_status: ProcessingStatus,
    pub settings: AppSettings,
    pub set_document: Callback<Option<PdfDocument>>,
    pub set_processing_status: Callback<ProcessingStatus>,
    pub update_settings: Callback<AppSettings>,
}

#[derive(Properties, PartialEq)]
pub struct AppStateProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(AppStateProvider)]
pub fn app_state_provider(props: &AppStateProviderProps) -> Html {
    let document_state = use_state(|| None);
    let processing_status_state = use_state(|| ProcessingStatus::Idle);
    let settings_state = use_state(AppSettings::default);

    let set_document = {
        let document_state = document_state.clone();
        Callback::from(move |doc: Option<PdfDocument>| {
            document_state.set(doc);
        })
    };

    let set_processing_status = {
        let processing_status_state = processing_status_state.clone();
        Callback::from(move |status: ProcessingStatus| {
            processing_status_state.set(status);
        })
    };

    let update_settings = {
        let settings_state = settings_state.clone();
        Callback::from(move |settings: AppSettings| {
            settings_state.set(settings);
        })
    };

    let context = AppState {
        document: (*document_state).clone(),
        processing_status: (*processing_status_state).clone(),
        settings: (*settings_state).clone(),
        set_document,
        set_processing_status,
        update_settings,
    };

    html! {
        <ContextProvider<AppState> context={context}>
            { for props.children.iter() }
        </ContextProvider<AppState>>
    }
}

#[hook]
pub fn use_app_state() -> Rc<AppState> {
    use_context::<AppState>()
        .map(Rc::new)
        .expect("AppState context not found")
}
