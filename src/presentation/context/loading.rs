use yew::prelude::*;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct LoadingContext {
    pub loading: bool,
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
    let loading_state = use_state(|| false);
    let message_state = use_state(|| None::<String>);

    let set_loading = {
        let loading_state = loading_state.clone();
        let message_state = message_state.clone();
        Callback::from(move |(is_loading, message): (bool, Option<String>)| {
            loading_state.set(is_loading);
            message_state.set(message);
        })
    };

    let context = LoadingContext {
        loading: *loading_state,
        message: (*message_state).clone(),
        set_loading,
    };

    html! {
        <ContextProvider<LoadingContext> context={context}>
            { for props.children.iter() }
            if *loading_state {
                <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
                    <div class="bg-white dark:bg-gray-800 p-6 rounded-lg shadow-xl">
                        <div class="animate-spin rounded-full h-12 w-12 border-4 border-blue-500 border-t-transparent"></div>
                        if let Some(message) = &*message_state {
                            <p class="mt-4 text-center text-gray-600 dark:text-gray-300">{ message }</p>
                        }
                    </div>
                </div>
            }
        </ContextProvider<LoadingContext>>
    }
}

#[hook]
pub fn use_loading() -> Rc<LoadingContext> {
    use_context::<LoadingContext>()
        .map(Rc::new)
        .expect("LoadingContext not found")
}
