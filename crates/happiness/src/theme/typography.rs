use std::collections::HashMap;
/// Typography provides

use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use yew::html::IntoPropValue;
use crate::style::Size;

/// The level for typography
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TypographyLevel {
    H1,
    H2,
    H3,
    H4,
    Title {
        size: Size,
    },
    Body {
        size: Size,
    },
    Custom(String)
}

impl Display for TypographyLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TypographyLevel::H1 => {
                write!(f, "h1")
            }
            TypographyLevel::H2 => {write!(f, "h2")
            }
            TypographyLevel::H3 => {write!(f, "h3")}
            TypographyLevel::H4 => {write!(f, "h4")}
            TypographyLevel::Title { size } => {write!(f, "title-{size}")}
            TypographyLevel::Body { size } => {write!(f, "body-{size}")}
            TypographyLevel::Custom(s) => { write!(f, "{s}")}
        }
    }
}

impl IntoPropValue<TypographyLevel> for &str {
    fn into_prop_value(self) -> TypographyLevel {
        TypographyLevel::from(self)
    }
}

impl From<&str> for TypographyLevel {
    fn from(value: &str) -> Self {
        match value {
            "h1" => TypographyLevel::H1,
            "h2" => TypographyLevel::H2,
            "h3" => TypographyLevel::H3,
            "h4" => TypographyLevel::H4,
            title if title.starts_with("title-") => {
                let size = title.strip_prefix("title-").unwrap();
                let size= IntoPropValue::<Size>::into_prop_value(size);
                TypographyLevel::Title { size }
            },
            body if body.starts_with("body-") => {
                let size = body.strip_prefix("body-").unwrap();
                let size= IntoPropValue::<Size>::into_prop_value(size);
                TypographyLevel::Body { size }
            },
            other => TypographyLevel::Custom(other.to_string())
        }
    }
}

impl Default for TypographyLevel {
    fn default() -> Self {
        TypographyLevel::Body { size: Size::Md }
    }
}

/// Provides the scale details for typography, giving weights, sizes, and margins for each level
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct TypographyScale {
    /// The scale details for given levels
    #[serde(flatten)]
    pub levels: HashMap<TypographyLevel, LevelScale>,
}

impl TypographyScale {

    /// Creates a new scale from
    pub fn new<I: IntoIterator<Item=(TypographyLevel, LevelScale)>>(levels: I) -> Self {
        Self { levels: levels.into_iter().collect() }
    }
}


/// Details for a specific level within the [`TypographyScale`](TypographyScale)
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct LevelScale {


}
