//! Theme context

use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use cfg_if::cfg_if;
use gloo::console::{console, console_dbg, info};
use gloo::utils::document;
use stylist::ast::ToStyleStr;
use stylist::manager::StyleManager;
use stylist::yew::styled_component;
use wasm_bindgen::JsCast;
use web_sys::{HtmlStyleElement, Node};
use yew::{function_component, html, use_effect_with, Children, Html, Properties, UseStateHandle, use_state_eq};

use crate::theme::baseline::baseline;
use crate::theme::theme_mode::ThemeMode;
use crate::theme::{hooks, Theme};
use crate::{Error, Sx};

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
    pub(crate) fn mount_wasm(
        &self,
        theme: &Theme,
        mode: &ThemeMode,
        to_mount: Sx,
    ) -> Result<(), crate::Error> {
        let document = document();
        let container = self.container().ok_or(Error::Web(None))?;

        (|| {
            let css = to_mount.to_css(&mode, theme);
            let style_element = document.create_element("style")?;
            let theme_name = format!("theme-{}-main", theme.prefix);
            style_element.set_attribute("data-style", &theme_name)?;
            style_element.set_text_content(Some(&css.to_style_str(None)));

            let list = container.child_nodes();
            let len = list.length();
            let mut existing: Option<Node> = None;
            for i in 0..len {
                if let Some(child) = list.get(i) {
                    if let Some(style_element) = child.dyn_ref::<HtmlStyleElement>() {
                        if style_element.get_attribute("data-style").as_ref() == Some(&theme_name) {
                            existing = Some(child);
                            break;
                        }
                    }
                }
            }

            if let Some(ref existing) = existing {
                container.replace_child(&style_element, existing)?;
            } else {
                container.append_child(&style_element)?;
            }


            Ok(())
        })()
        .map_err(|e| Error::Web(Some(e)))
    }
    pub fn mount(&self, theme: &Theme, mode: &ThemeMode, to_mount: Sx) -> Result<(), crate::Error> {
        cfg_if! {
            if #[cfg(target_arch="wasm32")] {
                self.mount_wasm(theme, mode, to_mount)
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

#[derive(Debug, Clone, PartialEq)]
pub struct ThemeModeContext {
    ctx: UseStateHandle<ThemeMode>
}

impl ThemeModeContext {
    pub fn new(ctx: UseStateHandle<ThemeMode>) -> Self {
        Self { ctx }
    }
}


impl Deref for ThemeModeContext {
    type Target = UseStateHandle<ThemeMode>;

    fn deref(&self) -> &Self::Target {
        &self.ctx
    }
}

impl DerefMut for ThemeModeContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ctx
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
    let mode = ThemeModeContext::new(use_state_eq(|| ThemeMode::System.detect()));


    html! {
            <yew::ContextProvider<ThemeContext> context={theme_state}>
                <yew::ContextProvider<ThemeModeContext> context={mode}>
                    <yew::ContextProvider<StyleManagerContext> context={manager}>
                        {for props.children.iter()}
                    </yew::ContextProvider<StyleManagerContext>>
                </yew::ContextProvider<ThemeModeContext>>
            </yew::ContextProvider<ThemeContext>>
    }
}

#[function_component]
pub fn CssBaseline() -> Html {
    let theme = hooks::use_theme();
    let style_manager: StyleManagerContext = hooks::use_style_manager();
    let (mode, ..) = hooks::use_mode();

    use_effect_with((theme, mode), move |(theme, mode)| {
        info!(format!("mounting {:#?}", theme));
        style_manager
            .mount(theme, &mode, baseline(theme, &mode))
            .expect("could not mount sx");
    });

    html! {}
}
