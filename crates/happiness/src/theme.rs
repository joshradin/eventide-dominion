//! Used for theming

use std::collections::HashMap;
use std::ops::Deref;

use dark_light::Mode;
use gloo::utils::document;
use once_cell::sync::Lazy;
use yew::Properties;

use crate::{theme, Error, Sx};
pub use color::Color;

use crate::theme::palette::Palette;

mod color;

pub mod baseline;
pub mod context;
pub mod hooks;
pub mod palette;
pub mod serde;
pub mod sx;

/// The theme kind
#[derive(Debug, Clone, PartialEq, Default)]
pub enum ThemeMode {
    /// Dark mode
    Dark,
    /// Light mode
    Light,
    /// Follow the system
    #[default]
    System,
}

impl ThemeMode {
    /// Detects system mode if possible, but only has effect if
    /// the mode is System
    pub fn detect(self) -> ThemeMode {
        match self {
            ThemeMode::System => match dark_light::detect() {
                Mode::Dark => ThemeMode::Dark,
                Mode::Light => ThemeMode::Light,
                Mode::Default => ThemeMode::Light,
            },
            other => other,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    pub prefix: String,
    palettes: HashMap<String, Palette>,
}

static DEFAULT_THEME: Lazy<Theme> = Lazy::new(|| {
    serde::from_str(include_str!("./theme/theme.json")).expect("could not read default theme")
});

impl Default for Theme {
    fn default() -> Self {
        DEFAULT_THEME.clone()
    }
}

impl Theme {
    pub fn new() -> Self {
        Self::with_prefix("happy")
    }

    pub fn with_prefix(prefix: impl AsRef<str>) -> Self {
        Self {
            prefix: prefix.as_ref().to_string(),
            palettes: Default::default(),
        }
    }

    /// Get a palette by name
    pub fn get_palette(&self, name: &str) -> Option<&Palette> {
        self.palettes.get(name)
    }

    /// Get a palette by name
    pub fn palette_mut(&mut self, name: &str) -> Option<&mut Palette> {
        self.palettes.get_mut(name)
    }

    /// Insert a palette into the theme
    pub fn insert_palette(&mut self, name: impl AsRef<str>, palette: Palette) {
        let _ = self.palettes.insert(name.as_ref().to_string(), palette);
    }

    /// Creates a new palette if not yet present, and returns a mutable reference to it.
    pub fn palette(&mut self, name: impl AsRef<str>) -> &mut Palette {
        self.palettes.entry(name.as_ref().to_string()).or_default()
    }

    pub fn var_name(&self, palette: &str, selector: &str) -> String {
        format!("--{}-palette-{palette}-{selector}", self.prefix)
    }

    /// Gets all palettes
    pub fn palettes(&self) -> impl Iterator<Item = (&str, &Palette)> {
        self.palettes.iter().map(|(key, value)| (&**key, value))
    }
}
