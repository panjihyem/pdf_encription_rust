pub mod theme;
pub mod language;
pub mod error;
pub mod loading;
pub mod progress;

pub use theme::{ThemeContext, ThemeProvider, use_theme};
pub use language::{LanguageContext, LanguageProvider, use_language};
pub use error::{ErrorContext, ErrorProvider, use_error};
pub use progress::{ProgressContext, ProgressProvider, use_progress};

use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct LoadingState {
    pub is_loading: bool,
    pub message: Option<String>,
}

impl Default for LoadingState {
    fn default() -> Self {
        Self {
            is_loading: false,
            message: None,
        }
    }
}

#[derive(Clone)]
pub struct LoadingContext {
    pub is_loading: bool,
    pub message: Option<String>,
    pub set_loading: Callback<(bool, Option<String>)>,
}

#[derive(Properties, PartialEq)]
pub struct LoadingProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(LoadingProvider)]
pub fn loading_provider(props: &LoadingProviderProps) -> Html {
    let loading_state = use_state(LoadingState::default);
    
    let loading_context = LoadingContext {
        is_loading: loading_state.is_loading,
        message: loading_state.message.clone(),
        set_loading: Callback::from({
            let loading_state = loading_state.clone();
            move |(is_loading, message): (bool, Option<String>)| {
                loading_state.set(LoadingState {
                    is_loading,
                    message,
                });
            }
        }),
    };

    html! {
        <ContextProvider<LoadingContext> context={loading_context}>
            { for props.children.iter() }
        </ContextProvider<LoadingContext>>
    }
}

pub use loading::{LoadingContext, LoadingProvider, use_loading};
