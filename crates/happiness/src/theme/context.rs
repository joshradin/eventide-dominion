//! Theme context

use cfg_if::cfg_if;
use gloo::console::console_dbg;
use gloo::utils::document;
use std::ops::Deref;
use std::rc::Rc;
use stylist::ast::ToStyleStr;
use stylist::css;

use crate::{sx, Error, Sx};
use stylist::manager::StyleManager;
use stylist::yew::styled_component;
use yew::{function_component, html, use_effect_with, Children, Html, Properties, UseStateHandle};

use crate::theme::baseline::baseline;
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

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn mount_wasm(&self, theme: &Theme, to_mount: Sx) -> Result<(), crate::Error> {
        let document = document();
        let container = self.container().ok_or(Error::Web(None))?;

        (|| {
            let css = to_mount.to_css(&ThemeMode::System, theme);
            let style_element = document.create_element("style")?;
            style_element.set_attribute("data-style", &format!("theme-{}-main", theme.prefix))?;
            style_element.set_text_content(Some(&css.to_style_str(None)));

            console_dbg!("creating style element: {:#?}", style_element);

            container.append_child(&style_element)?;
            Ok(())
        })()
        .map_err(|e| Error::Web(Some(e)))
    }
    pub fn mount(&self, theme: &Theme, to_mount: Sx) -> Result<(), crate::Error> {
        cfg_if! {
            if #[cfg(target_arch="wasm32")] {
                self.mount_wasm(theme, to_mount)
            } else {
                Err(Error::MountingUnsupported)
            }
        }
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
    let style_manager = hooks::use_style_manager();

    use_effect_with(theme, move |theme| {
        style_manager
            .mount(theme, baseline(theme, &ThemeMode::System.detect()))
            .expect("could not mount sx");
    });

    html! {}
}
