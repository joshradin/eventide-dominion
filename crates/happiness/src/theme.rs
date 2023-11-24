//! Used for theming

use std::collections::HashMap;
use std::ops::Deref;

use once_cell::sync::Lazy;
use yew::Properties;

pub use color::Color;

use crate::theme::palette::Palette;

pub mod color;

pub mod baseline;
pub mod context;
pub mod gradient;
pub mod hooks;
pub mod palette;
pub mod serde;
pub mod sx;
pub mod theme_mode;

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

    pub fn palette_var(&self, palette: &str, selector: &str) -> String {
        format!("--{}-palette-{palette}-{selector}", self.prefix)
    }

    pub fn class_var(&self, class: &str, var_name: &str) -> String {
        format!("--{}-{class}-{var_name}", self.prefix)
    }

    /// Gets all palettes
    pub fn palettes(&self) -> impl Iterator<Item = (&str, &Palette)> {
        self.palettes.iter().map(|(key, value)| (&**key, value))
    }
}
