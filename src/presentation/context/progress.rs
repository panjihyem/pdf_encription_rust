use yew::prelude::*;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct ProgressContext {
    pub progress: f32,
    pub message: Option<String>,
    pub set_progress: Callback<(f32, Option<String>)>,
}

#[derive(Properties, PartialEq)]
pub struct ProgressProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(ProgressProvider)]
pub fn progress_provider(props: &ProgressProviderProps) -> Html {
    let progress_state = use_state(|| 0.0);
    let message_state = use_state(|| None::<String>);

    let set_progress = {
        let progress_state = progress_state.clone();
        let message_state = message_state.clone();
        Callback::from(move |(progress, message): (f32, Option<String>)| {
            progress_state.set(progress.clamp(0.0, 100.0));
            message_state.set(message);
        })
    };

    let context = ProgressContext {
        progress: *progress_state,
        message: (*message_state).clone(),
        set_progress,
    };

    html! {
        <ContextProvider<ProgressContext> context={context}>
            { for props.children.iter() }
            if *progress_state > 0.0 && *progress_state < 100.0 {
                <div class="fixed bottom-0 left-0 right-0 h-1 bg-gray-200">
                    <div
                        class="h-full bg-blue-500 transition-all duration-300 ease-out"
                        style={format!("width: {}%", *progress_state)}
                    />
                </div>
                if let Some(message) = &*message_state {
                    <div class="fixed bottom-2 left-1/2 transform -translate-x-1/2 bg-gray-800 text-white px-4 py-2 rounded-md text-sm">
                        {message}
                    </div>
                }
            }
        </ContextProvider<ProgressContext>>
    }
}

#[hook]
pub fn use_progress() -> Rc<ProgressContext> {
    use_context::<ProgressContext>()
        .map(Rc::new)
        .expect("ProgressContext not found")
}
