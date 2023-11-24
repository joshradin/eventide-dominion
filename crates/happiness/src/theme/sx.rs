//! Contains the definition of the `Sx` type and the `sx!` macro
//!
//!
use std::fmt::{Debug, Formatter};
use std::ops::Index;
use std::str::FromStr;
use std::sync::Arc;

use cssparser::ToCss;
use gloo::history::query::FromQuery;
use heck::{ToKebabCase, ToTrainCase};
use indexmap::map::Entry;
use indexmap::IndexMap;
use stylist::ast::{Sheet, ToStyleStr};
use stylist::Style;
use yew::Classes;

pub use crate::theme::sx;
use crate::theme::sx::sx_to_css::sx_to_css;
use crate::theme::sx::sx_value_parsing::{parse_sx_value, ParseSxValueError};
use crate::theme::theme_mode::ThemeMode;
use crate::theme::{Color, Theme};

mod sx_to_css;
mod sx_value_parsing;

/// Contains CSS definition with some customization
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Sx {
    props: IndexMap<String, SxValue>,
}

impl Sx {
    /// Sets a css property
    pub fn insert<K: AsRef<str>, V: Into<SxValue>>(&mut self, key: K, value: V) {
        self.props.insert(key.as_ref().to_string(), value.into());
    }

    /// Merges this Sx with another Sx. Uses the left's values for conflicting keys.
    pub fn merge(self, other: Self) -> Self {
        let mut sx = self;

        for (prop, value) in other.props {
            match sx.props.entry(prop) {
                Entry::Occupied(mut occ) => {
                    if let SxValue::Nested(old_sx) = occ.get_mut() {
                        if let SxValue::Nested(sx) = value {
                            *old_sx = old_sx.clone().merge(sx);
                        }
                    }
                }
                Entry::Vacant(v) => {
                    v.insert(value);
                }
            }
        }

        sx
    }

    pub fn to_css(self, mode: &ThemeMode, theme: &Theme) -> Sheet {
        let css = sx_to_css(self, mode, theme, None).expect("invalid sx");
        Sheet::from_str(&css).unwrap()
    }

    /// Gets the properties set in this sx
    pub fn properties(&self) -> impl IntoIterator<Item = &str> {
        self.props.keys().map(|s| s.as_ref())
    }
}

impl Index<&str> for Sx {
    type Output = SxValue;

    fn index(&self, index: &str) -> &Self::Output {
        &self.props[index]
    }
}

impl From<SxRef> for Classes {
    fn from(value: SxRef) -> Self {
        Classes::from(value.style)
    }
}

#[derive(Debug, PartialEq, Clone)]
#[doc(hidden)]
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

/// Creates [`Sx`][Sx] instances
#[macro_export]
macro_rules! sx {
    (
        $($json:tt)*
    ) => {
        $crate::sx_internal!({ $($json)* })
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! sx_internal {

    // TT parser for objects

    // done
    (@object $object:ident () () ()) => {
    };

    // Insert the current entry followed by trailing comma.
    (@object $object:ident [$key:ident] ($value:expr) , $($rest:tt)*) => {
        let _ = $object.insert((stringify!($key)).trim(), sx_internal!($value));
        sx_internal!(@object $object () ($($rest)*) ($($rest)*));
    };

    // Insert the current entry followed by trailing comma.
    (@object $object:ident [$($key:tt)+] ($value:expr) , $($rest:tt)*) => {
        let _ = $object.insert(($($key)+), $value);
        sx_internal!(@object $object () ($($rest)*) ($($rest)*));
    };



     // Next value is a map.
    (@object $object:ident ($($key:tt)+) (: {$($map:tt)*} $($rest:tt)*) $copy:tt) => {
       sx_internal!(@object $object [$($key)+] (sx_internal!({$($map)*})) $($rest)*);
    };

     // Next value is a callback
    (@object $object:ident ($($key:tt)+) (: |$theme:ident| $func:expr , $($rest:tt)*) $copy:tt) => {
       sx_internal!(@object $object [$($key)+] (sx_internal!(|$theme| $func)) $($rest)*);
    };

    // Next value is a callback with no rest
    (@object $object:ident ($($key:tt)+) (: |$theme:ident| $func:expr) $copy:tt) => {
       sx_internal!(@object $object [$($key)+] (sx_internal!(|$theme| $func)));
    };

    // Next value is an expression followed by comma.
    (@object $object:ident ($($key:tt)+) (: $value:expr , $($rest:tt)*) $copy:tt) => {
        sx_internal!(@object $object [$($key)+] (sx_internal!($value)) , $($rest)*);
    };

    // Last value is an expression with no trailing comma.
    (@object $object:ident ($($key:tt)+) (: $value:expr) $copy:tt) => {
        sx_internal!(@object $object [$($key)+] ( sx_internal!($value) ) );
    };

     // Insert the last entry without trailing comma.
    (@object $object:ident [$key:ident] ($value:expr)) => {
        let _ = $object.insert((stringify!($key)).trim(), sx_internal!($value));
    };

     // Insert the last entry without trailing comma.
    (@object $object:ident [$($key:tt)+] ($value:expr)) => {
        let _ = $object.insert(($($key)+), sx_internal!($value));
    };


    // Key is fully parenthesized. This avoids clippy double_parens false
    // positives because the parenthesization may be necessary here.
    (@object $object:ident () (($key:expr) : $($rest:tt)*) $copy:tt) => {
        sx_internal!(@object $object ($key) (: $($rest)*) (: $($rest)*));
    };

    // Refuse to absorb colon token into key expression.
    (@object $object:ident ($($key:tt)*) (: $($unexpected:tt)+) $copy:tt) => {
        compile_error!("unexpected colon")
    };

    // Munch a token into the current key.
    (@object $object:ident ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => {
        sx_internal!(@object $object ($($key)* $tt) ($($rest)*) ($($rest)*));
    };


    // main implementation
    ({}) => {
        crate::theme::sx::Sx::default()
    };

    ({ $($tt:tt)+ }) => {
        {
            use $crate::theme::sx::*;
            use $crate::{sx, sx_internal};

            let mut sx: Sx = Sx::default();
            sx_internal!(@object sx () ($($tt)+) ($($tt)+));
            sx
        }
    };

    (|$theme:ident| $expr:expr) => {
        SxValue::Callback(FnSxValue::new(|$theme| $expr))
    };


    ($expr:expr) => {
        SxValue::try_from($expr).expect("could not create sxvalue")
    };
}

/// A style ref can be used as a css class
#[derive(Debug, Clone)]
pub struct SxRef {
    style: Style,
}

impl SxRef {
    pub(crate) fn new(style: Style) -> Self {
        Self { style }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_sx_with_macro() {
        let sx = sx! {
            width: "123.5%",
            p: "background.body",
        };
        assert_eq!(
            sx["p"],
            SxValue::ThemeToken {
                palette: "background".to_string(),
                selector: "body".to_string()
            }
        )
    }

    #[test]
    fn to_css() {
        let theme = Theme::default();

        let sx = sx! {
            padding: "15px",
            color: "background.body"
        };

        let style = sx.to_css(&ThemeMode::default(), &theme);
        println!("style: {style:#?}");
    }
}
