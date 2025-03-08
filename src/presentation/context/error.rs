use yew::prelude::*;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct ErrorContext {
    pub error: Option<String>,
    pub set_error: Callback<Option<String>>,
    pub clear_error: Callback<()>,
}

#[derive(Properties, PartialEq)]
pub struct ErrorProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(ErrorProvider)]
pub fn error_provider(props: &ErrorProviderProps) -> Html {
    let error_state = use_state(|| None::<String>);

    let set_error = {
        let error_state = error_state.clone();
        Callback::from(move |error: Option<String>| {
            if let Some(err) = &error {
                log::error!("Application error: {}", err);
            }
            error_state.set(error);
        })
    };

    let clear_error = {
        let error_state = error_state.clone();
        Callback::from(move |_| error_state.set(None))
    };

    let context = ErrorContext {
        error: (*error_state).clone(),
        set_error,
        clear_error,
    };

    html! {
        <ContextProvider<ErrorContext> context={context}>
            { for props.children.iter() }
        </ContextProvider<ErrorContext>>
    }
}

#[hook]
pub fn use_error() -> Rc<ErrorContext> {
    use_context::<ErrorContext>()
        .map(Rc::new)
        .expect("ErrorContext not found")
}
