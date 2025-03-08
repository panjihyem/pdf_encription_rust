use yew::prelude::*;
use crate::presentation::context::LoadingState;

#[function_component(LoadingIndicator)]
pub fn loading_indicator() -> Html {
    let loading_state = use_context::<LoadingState>().expect("No LoadingState found");

    if !loading_state.is_loading {
        return html! {};
    }

    html! {
        <div class="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full flex items-center justify-center">
            <div class="relative p-8 bg-white dark:bg-gray-800 rounded-lg shadow-xl">
                <div class="flex flex-col items-center">
                    <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500"></div>
                    <p class="mt-4 text-gray-700 dark:text-gray-300">
                        {loading_state.message.clone().unwrap_or_else(|| "Loading...".to_string())}
                    </p>
                </div>
            </div>
        </div>
    }
}
