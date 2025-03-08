use yew::prelude::*;
use crate::domain::entities::Theme;
use crate::infrastructure::storage::StorageService;

#[derive(Clone, Debug, PartialEq)]
pub struct ThemeContext {
    pub theme: Theme,
    pub set_theme: Callback<Theme>,
}

#[derive(Properties, PartialEq)]
pub struct ThemeProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(ThemeProvider)]
pub fn theme_provider(props: &ThemeProviderProps) -> Html {
    let storage = use_memo((), |_| {
        StorageService::new().expect("Failed to initialize storage service")
    });

    let theme_state = use_state(|| storage.get_theme().unwrap_or_default());
    let theme = *theme_state;

    let set_theme = {
        let theme_state = theme_state.clone();
        let storage = storage.clone();
        Callback::from(move |new_theme: Theme| {
            storage.set_theme(&new_theme);
            theme_state.set(new_theme);
        })
    };

    let context = ThemeContext {
        theme,
        set_theme,
    };

    let theme_class = match theme {
        Theme::Light => "",
        Theme::Dark => "dark",
        Theme::System => "system",
    };

    html! {
        <ContextProvider<ThemeContext> context={context}>
            <div class={classes!("min-h-screen", theme_class)}>
                { for props.children.iter() }
            </div>
        </ContextProvider<ThemeContext>>
    }
}

#[hook]
pub fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>().expect("ThemeContext not found")
}
