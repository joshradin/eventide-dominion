//! Used for theming

mod color;

use std::collections::HashMap;
pub use color::Color;
use gloo::utils::document;
use std::ops::Deref;
use dark_light::Mode;
use yew::{function_component, hook, html, Children, Html, Properties};
use crate::theme::palette::Palette;

pub mod context;
pub mod sx;
pub mod palette;

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
            ThemeMode::System => {
                match dark_light::detect() {
                    Mode::Dark => {
                        ThemeMode::Dark
                    }
                    Mode::Light => {
                        ThemeMode::Light
                    }
                    Mode::Default => {
                        ThemeMode::Light
                    }
                }
            }
            other => other
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    prefix: String,
    pub mode: ThemeMode,
    palettes: HashMap<String, Palette>
}

impl Default for Theme {
    fn default() -> Self {
        let mut theme = Theme::new();

        let mut common = theme.palette("common");
        common.insert_constant("white", Color::hex_code(0xFFFFFF));
        common.insert_constant("black", Color::hex_code(0x000000));


        {
            let mut background = Palette::new();
            background.insert_by_mode(
                "body",
                Color::Var {
                    name: theme.var_name("common", "black"),
                    fallback: Some(Box::new(Color::hex_code(0x000000))),
                },
                Color::Var {
                    name: theme.var_name("common", "white"),
                    fallback: Some(Box::new(Color::hex_code(0xFFFFFF))),
                }
            );


            theme.insert_palette("background", background);
        }


        theme
    }
}

impl Theme {
    pub fn new() -> Self {
        Self::with_prefix("happy")
    }

    pub fn with_prefix(prefix: impl AsRef<str>) -> Self {
        Self {
            prefix: prefix.as_ref().to_string(),
            mode: Default::default(),
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
        self.palettes.entry(name.as_ref().to_string())
            .or_default()

    }

    pub fn var_name(&self, palette: &str, selector: &str) -> String {
        format!("--{}-palette-{palette}-{selector}", self.prefix)
    }

    /// Gets all palettes
    pub fn palettes(&self) -> impl Iterator<Item=(&str, &Palette)> {
        self.palettes.iter().map(|(key, value)| {
            (&**key,
            value)
        })
    }
}
