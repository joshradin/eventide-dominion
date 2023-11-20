//! Theme context

use crate::theme::{Theme, ThemeMode};
use std::ops::Deref;
use stylist::yew::styled_component;
use yew::{function_component, hook, html, Children, Html, Properties, UseStateHandle};

/// The theme context
#[derive(Debug, Clone)]
pub struct ThemeContext {
    inner: UseStateHandle<Theme>,
}

impl ThemeContext {
    pub(crate) fn new(inner: UseStateHandle<Theme>) -> Self {
        Self { inner }
    }

    /// Modifies the theme
    pub fn modify<F: FnOnce(&mut Theme)>(&self, cb: F) {
        let mut theme: Theme = (*self.inner).clone();
        cb(&mut theme);
        self.inner.set(theme);
    }

    /// Set the theme mode
    pub fn set_mode(&self, mode: ThemeMode) {
        self.modify(|theme| {
            theme.mode = mode;
        })
    }
}

impl Deref for ThemeContext {
    type Target = Theme;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

impl PartialEq for ThemeContext {
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

/// Use a theme
#[hook]
pub fn use_theme() -> Theme {
    let theme = yew::use_context::<Theme>();
    theme.unwrap_or_else(Theme::default)
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ThemeProviderProps {
    #[prop_or_default]
    pub theme: Theme,
    #[prop_or_default]
    pub children: Children,
}

#[styled_component]
pub fn ThemeProvider(props: &ThemeProviderProps) -> Html {
    let theme_state = ThemeContext::new(yew::use_state_eq(|| props.theme.clone()));

    let theme = yew::use_memo(theme_state.clone(), |theme| {
        
    });

    html! {
        <yew::ContextProvider<ThemeContext> context={theme_state}>
            {for props.children.iter()}
        </yew::ContextProvider<ThemeContext>>
    }
}
