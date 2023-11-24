use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use std::str::FromStr;
use crate::Sx;
use crate::theme::{Color, Theme};
use crate::theme::sx::sx_value_parsing::{parse_sx_value, ParseSxValueError};

/// An sx value
#[derive(Debug, PartialEq, Clone)]
pub enum SxValue {
    Integer(i32),
    Float(f32),
    Percent(f32),
    FloatDimension {
        value: f32,
        unit: String,
    },
    Dimension {
        value: i32,
        unit: String,
    },
    CssLiteral(String),
    String(String),
    Color(Color),
    ThemeToken {
        palette: String,
        selector: String,
    },
    ClassVar {
        class: String,
        var: String,
        fallback: Option<Box<SxValue>>,
    },
    Callback(FnSxValue),
    Nested(Sx),
}

impl SxValue {
    pub fn var(class: &str, var: &str, fallback: impl Into<Option<SxValue>>) -> Self {
        Self::ClassVar {
            class: class.to_string(),
            var: var.to_string(),
            fallback: fallback.into().map(|fallback| Box::new(fallback)),
        }
    }

    pub fn to_css(self) -> Option<String> {
        Some(match self {
            SxValue::Integer(i) => {
                format!("{i}")
            }
            SxValue::Float(f) => {
                format!("{f}")
            }
            SxValue::Percent(p) => {
                format!("{}%", p * 100.0)
            }
            SxValue::FloatDimension { value, unit } => {
                format!("{value}{unit}")
            }
            SxValue::Dimension { value, unit } => {
                format!("{value}{unit}")
            }
            SxValue::CssLiteral(lit) => {
                format!("{lit}")
            }
            SxValue::String(s) => {
                format!("\"{s}\"")
            }
            SxValue::Color(c) => c.to_string(),
            _other => return None,
        })
    }
}

impl From<i32> for SxValue {
    fn from(value: i32) -> Self {
        Self::Integer(value)
    }
}

impl From<f32> for SxValue {
    fn from(value: f32) -> Self {
        Self::Float(value)
    }
}

impl From<&str> for SxValue {
    fn from(quoted_str: &str) -> Self {
        let split = quoted_str.split(".").collect::<Vec<_>>();
        if split.len() == 2 && !(split[0].trim().is_empty() || split[1].trim().is_empty()) {
            let palette = split[0].trim().to_string();
            let selector = split[1].trim().to_string();

            SxValue::ThemeToken { palette, selector }
        } else {
            quoted_str.parse().unwrap()
        }
    }
}

impl From<Sx> for SxValue {
    fn from(value: Sx) -> Self {
        Self::Nested(value)
    }
}

impl FromStr for SxValue {
    type Err = ParseSxValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_sx_value(s)
    }
}

/// An sx value derived from a function
#[derive(Clone)]
pub struct FnSxValue {
    id: u64,
    callback: Arc<dyn Fn(&Theme) -> SxValue>,
}

impl PartialEq for FnSxValue {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl FnSxValue {
    pub fn new<R, F: Fn(&Theme) -> R + 'static>(callback: F) -> Self
    where
        R: Into<SxValue>,
    {
        Self {
            id: rand::random(),
            callback: Arc::new(move |theme: &Theme| (callback)(theme).into()),
        }
    }

    pub fn apply(&self, theme: &Theme) -> SxValue {
        (self.callback)(theme)
    }
}

impl Debug for FnSxValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(&Theme) => SxValue")
    }
}
