//! Controls the general style and approach of components

use std::env::var;
use std::fmt::{Display, Formatter};
use strum::{AsRefStr, EnumIter, IntoEnumIterator};
use yew::html::{ImplicitClone, IntoPropValue};

/// The variant of this component to use
#[derive(Debug,Clone, Copy, Default, PartialEq, Eq, AsRefStr, EnumIter)]
#[strum()]
pub enum Variant {
    /// Plain theme
    #[default]
    Plain,
    /// Outlined theme
    Outlined,
    /// A softer theme
    Soft,
    /// A solid theme
    Solid
}

impl ImplicitClone for Variant {
}

impl IntoPropValue<Variant> for &str {
    fn into_prop_value(self) -> Variant {
        for variant in Variant::iter() {
            if variant.as_ref().to_lowercase() == self {
                return variant;
            }
        }
        panic!("{self:?} is not a known variant")
    }
}

impl Display for Variant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref().to_lowercase())
    }
}


/// The main color scheme of this component to use
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, AsRefStr, EnumIter)]
pub enum Color {
    #[default]
    Neutral,
    Primary,
    Success,
    Fatal,
    Warn,
}

impl ImplicitClone for Color {}

impl IntoPropValue<Color> for &str {
    fn into_prop_value(self) -> Color {
        for color in Color::iter() {
            if color.as_ref().to_lowercase() == self {
                return color;
            }
        }
        panic!("{self:?} is not a known color")
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref().to_lowercase())
    }
}
