use yew::prelude::*;
use yew_router::prelude::*;

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

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! {
            <pages::home::Home />
        },
        Route::Encrypt => html! {
            <pages::encrypt::Encrypt />
        },
        Route::Decrypt => html! {
            <pages::decrypt::Decrypt />
        },
        Route::Settings => html! {
            <pages::settings::Settings />
        },
        Route::NotFound => html! {
            <pages::not_found::NotFound />
        },
    }
}
