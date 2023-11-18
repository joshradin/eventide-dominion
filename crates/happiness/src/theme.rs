//! Used for theming



mod color;

use std::ops::Deref;
use gloo::utils::document;
use yew::{Children, function_component, hook, Html, html, Properties};
pub use color::Color;

pub mod sx;


/// The theme kind
#[derive(Debug, Clone, PartialEq, Default)]
pub enum ThemeMode {
    /// Dark mode
    Dark,
    /// Light mode
    #[default]
    Light,
    /// Follow the system
    System
}



#[derive(Debug, Clone, PartialEq, Default)]
pub struct Theme {
    pub mode: ThemeMode
}

/// Use a theme
#[hook]
pub fn use_theme() -> Theme  {
    let theme = yew::use_context::<Theme>();
    theme.unwrap_or_else(Theme::default)
}


#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ThemeProviderProps {
    #[prop_or_default]
    pub theme: Theme,
    #[prop_or_default]
    pub children: Children
}

#[function_component]
pub fn ThemeProvider(props: &ThemeProviderProps) -> Html {
    let theme_state = yew::use_state_eq(|| props.theme.clone());

    html! {
        <yew::ContextProvider<Theme> context={theme_state.deref().clone()}>
            {for props.children.iter()}
        </yew::ContextProvider<Theme>>
    }
}