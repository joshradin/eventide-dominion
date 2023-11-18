//! Contains the definition of the `Sx` type and the `sx!` macro
//!
//!
use std::str::FromStr;

use indexmap::IndexMap;
use nom::IResult;

/// Contains CSS definition with some customization
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Sx {
    props: IndexMap<String, SxValue>
}

impl Sx {
    /// Sets a css property
    pub fn insert<K: AsRef<str>, V: Into<SxValue>>(&mut self, key: K, value: V) {
        self.props.insert(key.as_ref().to_string(), value.into());
    }
}

#[derive(Debug, PartialEq, Clone)]
#[doc(hidden)]
pub enum SxValue {
    Integer(i64),
    Float(f64),
    Percent(f64),
    CssLiteral(String),
    Token {
        path: Vec<String>
    }
}

impl From<i64> for SxValue {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<f64> for SxValue {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl FromStr for SxValue {
    type Err = ParseSxValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

/// An error occurred while trying to parse this value
#[derive(Debug, thiserror::Error)]
pub enum ParseSxValueError {

}


impl TryFrom<&str> for SxValue {
    type Error = ParseSxValueError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

fn parse_sx_value(s: &str) -> IResult<&str, SxValue> {
   todo!()
}





/// Creates [`Sx`][Sx] instances
#[macro_export]
macro_rules! sx {
    (
        $($name:tt : $val:expr),* $(,)?
    ) => {
        $crate::sx_impl!($($name : $val),*)
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! sx_impl {
    (
        $($name:tt : $val:expr),* $(,)?
    ) => {
        {
            use $crate::theme::sx::{Sx, sx, SxValue};
            let mut emit = Sx::default();

            $(
                sx_impl!(@ &mut emit, $name: $val);
            )*

            emit
        }
    };
    (@
        $sx:expr, $prop:literal : $val:literal%
    ) => {
        {
            let mut sx: &mut Sx = $sx;
            let prop: &str = $prop;
            let value: SxValue = SxValue::try_from($val).expect("could not convert to sx value");

            sx.insert(prop, value);
        }
    };
    (@
        $sx:expr, $prop:literal : $val:expr
    ) => {
        {
            let mut sx: &mut Sx = $sx;
            let prop: &str = $prop;
            let value: SxValue = SxValue::try_from($val).expect("could not convert to sx value");

            sx.insert(prop, value);
        }
    };
    (@
        $sx:expr, $prop:ident : $val:expr
    ) => {
        {
            let mut sx: &mut Sx = $sx;
            let prop: &str = stringify!($prop);
            let value: SxValue = SxValue::try_from($val).expect("could not convert to sx value");

            sx.insert(prop, value);
        }
    };
}

pub use sx;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_sx_with_macro() {

        let sx = sx! {
            p: 123,
            "p": 1,
            "text-color": "red"
        };
        println!("{sx:#?}");
    }
}