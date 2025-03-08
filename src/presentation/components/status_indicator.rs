use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ProcessingStatus {
    Idle,
    Processing,
    Success,
    Error,
}

#[derive(Properties, PartialEq)]
pub struct StatusIndicatorProps {
    pub status: ProcessingStatus,
    pub message: String,
    #[prop_or_default]
    pub icon: Option<String>,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(StatusIndicator)]
pub fn status_indicator(props: &StatusIndicatorProps) -> Html {
    let status_classes = match props.status {
        ProcessingStatus::Idle => "text-gray-500 dark:text-gray-400",
        ProcessingStatus::Processing => "text-blue-500 dark:text-blue-400",
        ProcessingStatus::Success => "text-green-500 dark:text-green-400",
        ProcessingStatus::Error => "text-red-500 dark:text-red-400",
    };

    let icon = props.icon.clone().unwrap_or_else(|| {
        match props.status {
            ProcessingStatus::Idle => "⚪",
            ProcessingStatus::Processing => "🔄",
            ProcessingStatus::Success => "✅",
            ProcessingStatus::Error => "❌",
        }.to_string()
    });

    let container_classes = classes!(
        "flex",
        "items-center",
        "space-x-2",
        "p-2",
        "rounded-md",
        status_classes,
        props.class.clone()
    );

    html! {
        <div class={container_classes}>
            <span class="text-lg">{icon}</span>
            <span>{&props.message}</span>
        </div>
    }
}
