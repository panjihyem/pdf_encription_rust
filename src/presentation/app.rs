use yew::prelude::*;
use yew_router::prelude::*;

use crate::presentation::pages::{HomePage, EncryptPage, DecryptPage, SettingsPage, NotFoundPage};
use crate::presentation::components::LoadingIndicator;
use crate::presentation::context::LoadingProvider;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/encrypt")]
    Encrypt,
    #[at("/decrypt")]
    Decrypt,
    #[at("/settings")]
    Settings,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <LoadingProvider>
                    <div class="min-h-screen bg-gray-100 dark:bg-gray-900">
                        <nav class="bg-white dark:bg-gray-800 shadow">
                            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                                <div class="flex justify-between h-16">
                                    <div class="flex">
                                        <Link<Route> to={Route::Home} classes="flex items-center px-4 py-2 text-gray-700 dark:text-gray-200 hover:text-blue-600">
                                            {"Home"}
                                        </Link<Route>>
                                        <Link<Route> to={Route::Encrypt} classes="flex items-center px-4 py-2 text-gray-700 dark:text-gray-200 hover:text-blue-600">
                                            {"Encrypt PDF"}
                                        </Link<Route>>
                                        <Link<Route> to={Route::Decrypt} classes="flex items-center px-4 py-2 text-gray-700 dark:text-gray-200 hover:text-blue-600">
                                            {"Decrypt PDF"}
                                        </Link<Route>>
                                        <Link<Route> to={Route::Settings} classes="flex items-center px-4 py-2 text-gray-700 dark:text-gray-200 hover:text-blue-600">
                                            {"Settings"}
                                        </Link<Route>>
                                    </div>
                                </div>
                            </div>
                        </nav>

                        <main class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
                            <LoadingIndicator />
                            <Switch<Route> render={switch} />
                        </main>
                    </div>
                </LoadingProvider>
            </BrowserRouter>
        }
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage /> },
        Route::Encrypt => html! { <EncryptPage /> },
        Route::Decrypt => html! { <DecryptPage /> },
        Route::Settings => html! { <SettingsPage /> },
        Route::NotFound => html! { <NotFoundPage /> },
    }
}
