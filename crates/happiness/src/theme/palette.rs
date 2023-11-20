//! Contains palette

use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

use crate::theme::{Color, ThemeMode};

/// A palette contains an assortment of colors
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Palette {
    selector_to_colors: HashMap<String, ColorByMode>,
}

impl Palette {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_constant(&mut self, key: &str, val: Color) {
        let _ = self
            .selector_to_colors
            .insert(key.to_string(), ColorByMode::Constant(val));
    }

    pub fn insert_by_mode(&mut self, key: &str, dark: Color, light: Color) {
        let _ = self
            .selector_to_colors
            .insert(key.to_string(), ColorByMode::ModeBased { dark, light });
    }

    /// Gets all the selectors for this palette
    pub fn selectors(&self) -> impl Iterator<Item = &str> {
        self.selector_to_colors.keys().map(|s| &**s)
    }

    pub fn get<Q: Eq + Hash>(&self, selector: &Q, mode: &ThemeMode) -> Option<&Color>
    where
        String: Borrow<Q>,
    {
        let by_mode = self.selector_to_colors.get(selector)?;
        match by_mode {
            ColorByMode::Constant(c) => Some(c),
            ColorByMode::ModeBased { dark, light } => match mode.clone().detect() {
                ThemeMode::Dark => Some(dark),
                ThemeMode::Light => Some(light),
                ThemeMode::System => {
                    unreachable!()
                }
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum ColorByMode {
    Constant(Color),
    ModeBased { dark: Color, light: Color },
}
