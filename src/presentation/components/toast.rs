use std::time::Duration;
use yew::prelude::*;
use gloo::timers::callback::Timeout;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum ToastType {
    Success,
    Error,
    Info,
    Warning,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Toast {
    pub id: String,
    pub message: String,
    pub toast_type: ToastType,
    pub duration: Duration,
}

impl Toast {
    pub fn new(message: String, toast_type: ToastType) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            message,
            toast_type,
            duration: Duration::from_secs(5),
        }
    }

    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }
}

#[derive(Properties, PartialEq)]
pub struct ToastProps {
    pub toast: Toast,
    pub on_dismiss: Callback<String>,
}

#[function_component(ToastComponent)]
pub fn toast_component(props: &ToastProps) -> Html {
    let ToastProps { toast, on_dismiss } = props;
    let id = toast.id.clone();

    use_effect_with_deps(
        move |(id, on_dismiss, duration)| {
            let id = id.clone();
            let on_dismiss = on_dismiss.clone();
            let timeout = Timeout::new(duration.as_millis() as u32, move || {
                on_dismiss.emit(id);
            });
            move || {
                timeout.cancel();
            }
        },
        (id, on_dismiss.clone(), toast.duration),
    );

    let toast_classes = match toast.toast_type {
        ToastType::Success => "bg-green-500",
        ToastType::Error => "bg-red-500",
        ToastType::Info => "bg-blue-500",
        ToastType::Warning => "bg-yellow-500",
    };

    html! {
        <div class={classes!(
            "fixed",
            "bottom-4",
            "right-4",
            "p-4",
            "rounded-lg",
            "text-white",
            "shadow-lg",
            "flex",
            "items-center",
            "justify-between",
            "min-w-[300px]",
            "z-50",
            "animate-slide-in",
            toast_classes
        )}>
            <p class="mr-4">{ &toast.message }</p>
            <button
                class="hover:opacity-75 focus:outline-none"
                onclick={let on_dismiss = on_dismiss.clone();
                    let id = toast.id.clone();
                    move |_| on_dismiss.emit(id.clone())}
            >
                <svg class="w-4 h-4 fill-current" viewBox="0 0 20 20">
                    <path d="M10 8.586L2.929 1.515 1.515 2.929 8.586 10l-7.071 7.071 1.414 1.414L10 11.414l7.071 7.071 1.414-1.414L11.414 10l7.071-7.071-1.414-1.414L10 8.586z"/>
                </svg>
            </button>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ToastContainerProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(ToastContainer)]
pub fn toast_container(props: &ToastContainerProps) -> Html {
    let toasts = use_state(Vec::new);

    let add_toast = {
        let toasts = toasts.clone();
        Callback::from(move |toast: Toast| {
            let mut current = (*toasts).clone();
            current.push(toast);
            toasts.set(current);
        })
    };

    let remove_toast = {
        let toasts = toasts.clone();
        Callback::from(move |id: String| {
            let mut current = (*toasts).clone();
            current.retain(|t| t.id != id);
            toasts.set(current);
        })
    };

    html! {
        <>
            { for props.children.iter() }
            <div class="fixed bottom-4 right-4 flex flex-col gap-2 z-50">
                { for toasts.iter().map(|toast| {
                    html! {
                        <ToastComponent
                            key={toast.id.clone()}
                            toast={toast.clone()}
                            on_dismiss={remove_toast.clone()}
                        />
                    }
                }) }
            </div>
        </>
    }
}
