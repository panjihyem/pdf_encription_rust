use yew::prelude::*;
use yew_router::prelude::*;
use crate::presentation::routing::Route;
use crate::infrastructure::i18n::I18nService;

pub struct NotFound {
    i18n: I18nService,
}

impl Component for NotFound {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            i18n: I18nService::new().expect("Failed to initialize i18n service"),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="min-h-screen flex items-center justify-center bg-gray-50 dark:bg-gray-900">
                <div class="text-center">
                    <h1 class="text-6xl font-bold text-gray-900 dark:text-gray-100 mb-4">
                        { "404" }
                    </h1>
                    <p class="text-xl text-gray-600 dark:text-gray-300 mb-8">
                        { self.i18n.t("page_not_found") }
                    </p>
                    <Link<Route> 
                        to={Route::Home}
                        classes="bg-blue-500 hover:bg-blue-600 text-white font-semibold py-2 px-4 rounded"
                    >
                        { self.i18n.t("back_to_home") }
                    </Link<Route>>
                </div>
            </div>
        }
    }
}
