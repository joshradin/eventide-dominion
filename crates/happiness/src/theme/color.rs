use std::fmt::{Display, Formatter};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::combinator::{map_res, opt};
use nom::IResult;
use nom::sequence::tuple;

/// Html compatible
#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    /// A named CSS color
    Named(String),
    /// A hex color
    Hex(u32),
    /// Rgb color
    Rgb {
        r: u8,
        g: u8,
        b: u8,
    },
    /// Rgba color
    Rgba {
        r: u8,
        g: u8,
        b: u8,
        a: u8
    },
    /// hsl color
    Hsl {
        h: u16,
        s: u8,
        l: u8
    },
    /// hsla color
    Hsla {
        h: u16,
        s: u8,
        l: u8,
        a: u8
    }
}

/// HSL are constrained to `[0, 1]`
fn hsl_to_rgb(h: f64, s: f64, l: f64) -> [u8; 3] {
    let mut r: f64 =0.;
    let mut g: f64 =0.;
    let mut b: f64 =0.;

    if s == 0. {
        r = l;
        g = l;
        b = l;
    } else {
        let q = if l < 0.5{
            l * (1.+s)
        }  else {
            l + s -l * s
        };
        let p = 2.*l-q;
        r = hue_to_rgb(p, q, h + (1./3.));
        g = hue_to_rgb(p, q, h);
        b = hue_to_rgb(p, q, h - (1./3.));
    }
    [
        (r * 255.0).round() as u8,
        (g * 255.0).round() as u8,
        (b * 255.0).round() as u8,
    ]
}
fn hue_to_rgb(p: f64, q: f64, mut t: f64) -> f64 {
    if t < 0.0 {
        t =t + 1.;
    }
    if t > 1.0 {
        t = t - 1.;
    }
    if t < 1.0/6.0 {
        p + (q - p) * 6. * t
    } else if t < 0.5 {
        q
    } else if t < 2.0/3.0 {
        p + (q-p) * (2./3. - t) * 6.
    } else {
        p
    }
}

impl Color {

    /// Creates a new color with a name
    pub fn named<S : AsRef<str>>(name: S) -> Self {
        Self::Named(name.as_ref().to_string())
    }

    /// Creates a new color by hex
    pub fn hex_code(value: u32) -> Self {
        Self::Hex(value)
    }


}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Named(n) => {
                write!(f, "{n}")
            }
            Color::Hex(hex) => {
                write!(f, "#{hex:08X}")
            }
            Color::Rgb {
                r, g, b
            } => {
                write!(f, "rgb({r}, {g}, {b})")
            }
            Color::Rgba { r, g, b, a } => {
                write!(f, "rgba({r}, {g}, {b}, {a})")
            }
            Color::Hsl { h, s, l } => {
                write!(f, "hsla({h}, {s}, {l})")
            }
            Color::Hsla { h, s, l, a } => {
                write!(f, "hsla({h}, {s}, {l}, {a})")
            }
        }
    }
}

pub fn parse_color(color: &str) -> IResult<&str, Color> {
    alt(
        (
            parse_hex_color,
            )
    )
        (color)
}
fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}
fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}
fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(
        take_while_m_n(2, 2, is_hex_digit),
        from_hex
    )(input)
}
fn parse_hex_color(color: &str) -> IResult<&str, Color> {
    let (rest, _) = tag("#")(color)?;
    let (rest, (r, g, b, a)) = tuple((hex_primary, hex_primary, hex_primary, opt(hex_primary)))(rest)?;

    Ok((rest, match a {
        None => {
            Color::Rgb {
                r, g, b
            }
        }
        Some(a) => {
            Color::Rgba {
                r, g, b, a
            }
        }
    }))
}

#[cfg(test)]
mod tests {
    use nom::Finish;
    use crate::theme::color::{hsl_to_rgb, parse_color};

    #[test]
    fn parse_hex_color() {
        let hex = "#0f125f";
        let (_, parsed) = parse_color(hex).finish().unwrap();
        println!("{parsed}")
    }

    #[test]
    fn parse_hex_alpha_color() {
        let hex = "#01f1257f";
        let (_, parsed) = parse_color(hex).finish().unwrap();
        println!("{parsed}")
    }



    #[test]
    fn hsl_to_rgb_correctness() {
        let (h, s, l) = (126.0 / 360., 0.46, 0.63);
        let [r, g, b] = hsl_to_rgb(h, s, l);
        assert_eq!(r, 117);
        assert_eq!(g, 204);
        assert_eq!(b, 126);


    }
}