use yew::prelude::*;

// Loading Context
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
pub enum LoadingAction {
    StartLoading(Option<String>),
    StopLoading,
}

#[derive(Properties, Clone, PartialEq)]
pub struct LoadingProviderProps {
    #[prop_or_default]
    pub children: Children,
}

pub struct LoadingProvider {
    state: LoadingState,
}

impl Component for LoadingProvider {
    type Message = LoadingAction;
    type Properties = LoadingProviderProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            state: LoadingState::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LoadingAction::StartLoading(message) => {
                self.state.is_loading = true;
                self.state.message = message;
                true
            }
            LoadingAction::StopLoading => {
                self.state.is_loading = false;
                self.state.message = None;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <ContextProvider<LoadingState> context={self.state.clone()}>
                { ctx.props().children.clone() }
            </ContextProvider<LoadingState>>
        }
    }
}
