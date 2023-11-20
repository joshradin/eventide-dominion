//! Theme context

use std::ops::Deref;
use std::rc::Rc;
use stylist::css;

use stylist::manager::StyleManager;
use stylist::yew::styled_component;
use yew::{
    Children, function_component, html, Html, Properties, use_effect_with,
    UseStateHandle,
};
use crate::sx;

use crate::theme::{hooks, Theme, ThemeMode};

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

#[derive(Default, Debug, Clone, PartialEq)]
pub struct StyleManagerContext {
    manager: Rc<StyleManager>,
}

impl StyleManagerContext {
    pub fn new(manager: Rc<StyleManager>) -> Self {
        Self { manager }
    }
}

impl Deref for StyleManagerContext {
    type Target = StyleManager;

    fn deref(&self) -> &Self::Target {
        &*self.manager
    }
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
    let manager = StyleManagerContext::new(yew::use_memo(theme_state.clone(), |_| {
        StyleManager::builder()
            .prefix(theme_state.prefix.clone().into())
            .build()
            .expect("could not create style manager")
    }));

    html! {
            <yew::ContextProvider<ThemeContext> context={theme_state}>
                <yew::ContextProvider<StyleManagerContext> context={manager}>
                    {for props.children.iter()}
                </yew::ContextProvider<StyleManagerContext>>
            </yew::ContextProvider<ThemeContext>>
    }
}

#[function_component]
pub fn CssBaseline() -> Html {
    let theme = hooks::use_theme();

    use_effect_with(theme, |theme| {
        theme.mount(
            sx! {

            }
        )
    });

    html! {
        <></>
    }
}
