//! Allows loading of themes from json files

use std::io;
use std::io::Read;

use indexmap::IndexMap;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;

use crate::theme::color::hsl_to_rgb;
use crate::theme::palette::Palette;
use crate::theme::{Color, Theme};

/// Parses a theme from a reader
pub fn from_reader<R: Read>(reader: R) -> Result<Theme, io::Error> {
    let json: ThemeJson = serde_json::from_reader(reader)?;
    Ok(from_theme_json(json))
}

pub fn from_str(reader: &str) -> Result<Theme, io::Error> {
    let json: ThemeJson = serde_json::from_str(reader)?;
    Ok(from_theme_json(json))
}

static PALETTE_SELECTOR_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"^(?<palette>[a-zA-Z_]\w*)\.(?<selector>\w+)$"#).expect("could not create palette selector")
});

fn adjust_color(color: Color, theme: &Theme) -> Color {
    match color {
        Color::Var { var, fallback } if PALETTE_SELECTOR_REGEX.is_match(&var) => {
            let captures = PALETTE_SELECTOR_REGEX.captures(&var).expect("must pass here");
            let palette = &captures["palette"];
            let selector = &captures["selector"];

            Color::Var {
                var: theme.var_name(palette, selector),
                fallback,
            }
        }
        color => color
    }
}
fn from_theme_json(json: ThemeJson) -> Theme {
    let mut theme = json
        .prefix
        .map(|prefix| Theme::with_prefix(prefix))
        .unwrap_or_else(Theme::new);
    for (palette_name, def) in json.palettes {
        let mut palette = Palette::new();
        match def {
            PaletteJson::Range {
                light: min,
                dark: max,
            } => {
                let min = adjust_color(min.unwrap_or(Color::Hex(0xFFFFFF)), &theme)
                    .to_hsla()
                    .expect("#could not get HSLA value for min");
                println!("low: {min:?}");
                let max = adjust_color(max.unwrap_or(Color::Hex(0x000000)), &theme)
                    .to_hsla()
                    .expect("could not get HSLA value for max");
                println!("high: {max:?}");
                let h_step = (max[0] - min[0]) / 8.0;
                let s_step = (max[1] - min[1]) / 8.0;
                let l_step = (max[2] - min[2]) / 8.0;
                let a_step = (max[3] - min[3]) / 8.0;
                println!("diff: {:?}", [h_step, s_step, l_step, a_step]);
                for (selector, h, s, l, a) in (0..9).map(|i| (i, i as f32)).map(|(i, m)| {
                    (
                        (i + 1) * 100,
                        m * h_step + min[0],
                        m * s_step + min[1],
                        m * l_step + min[2],
                        m * a_step + min[3],
                    )
                }) {
                    let [r, g, b] = hsl_to_rgb(h, s, l);
                    println!("converted hsl({h}, {s}, {l}) to rgb({r}, {g}, {b})");
                    let a = (a * 255.0) as u8;

                    let str = if a == 0 {
                        format!("#{r:02X}{g:02X}{b:02X}")
                    } else {
                        format!("#{r:02X}{g:02X}{b:02X}{a:02X}")
                    };
                    let color = Color::CSSLiteral(str.to_string());
                    palette.insert_constant(&format!("{}", selector), color);
                }
            }
            PaletteJson::Preset { selectors } => {
                for (selector, color) in selectors {
                    match color {
                        SelectorJson::Const(c) => {
                            let c = adjust_color(c, &theme);
                            palette.insert_constant(&selector, c);
                        }
                        SelectorJson::DarkLight { dark, light } => {
                            let dark = adjust_color(dark, &theme);
                            let light = adjust_color(light, &theme);
                            palette.insert_by_mode(&selector, dark, light);
                        }
                    }
                }
            }
        }

        theme.insert_palette(palette_name, palette);
    }
    theme
}

#[derive(Debug, Deserialize)]
struct ThemeJson {
    prefix: Option<String>,
    palettes: IndexMap<String, PaletteJson>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum PaletteJson {
    Range {
        light: Option<Color>,
        dark: Option<Color>,
    },
    Preset {
        #[serde(flatten)]
        selectors: IndexMap<String, SelectorJson>,
    },
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum SelectorJson {
    Const(Color),
    DarkLight { dark: Color, light: Color },
}

#[cfg(test)]
mod tests {
    use crate::theme::serde::from_str;

    #[test]
    fn parse_theme_json() {
        let json = include_str!("./theme.json");
        let parsed = from_str(json).expect("could not parse");

        println!("parsed: {:#?}", parsed);
    }
}
