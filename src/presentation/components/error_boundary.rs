use yew::prelude::*;
use crate::infrastructure::i18n::I18nService;

#[derive(Properties, PartialEq)]
pub struct ErrorBoundaryProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub fallback: Option<Html>,
}

pub struct ErrorBoundary {
    error: Option<String>,
    i18n: I18nService,
}

pub enum ErrorBoundaryMsg {
    ClearError,
}

impl Component for ErrorBoundary {
    type Message = ErrorBoundaryMsg;
    type Properties = ErrorBoundaryProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            error: None,
            i18n: I18nService::new().expect("Failed to initialize i18n service"),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ErrorBoundaryMsg::ClearError => {
                self.error = None;
                true
            }
        }
    }

    fn catch(&mut self, error: std::error::Error) -> bool {
        log::error!("Error caught by boundary: {:?}", error);
        self.error = Some(error.to_string());
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(error) = &self.error {
            if let Some(fallback) = &ctx.props().fallback {
                fallback.clone()
            } else {
                html! {
                    <div class="min-h-screen flex items-center justify-center bg-gray-50 dark:bg-gray-900">
                        <div class="max-w-md w-full px-6 py-8 bg-white dark:bg-gray-800 shadow-lg rounded-lg">
                            <div class="text-center">
                                <h2 class="text-2xl font-bold text-red-600 dark:text-red-400 mb-4">
                                    { self.i18n.t("error_occurred") }
                                </h2>
                                <p class="text-gray-600 dark:text-gray-300 mb-6">
                                    { error }
                                </p>
                                <button
                                    onclick={ctx.link().callback(|_| ErrorBoundaryMsg::ClearError)}
                                    class="bg-blue-500 hover:bg-blue-600 text-white font-semibold py-2 px-4 rounded"
                                >
                                    { self.i18n.t("try_again") }
                                </button>
                            </div>
                        </div>
                    </div>
                }
            }
        } else {
            html! {
                <>{ for ctx.props().children.iter() }</>
            }
        }
    }
}
