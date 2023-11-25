//! Allows loading of themes from json files

use std::io;
use std::io::Read;

use indexmap::IndexMap;
use serde::Deserialize;

use crate::theme::gradient::Gradient;
use crate::theme::palette::Palette;
use crate::theme::{Color, Theme, PALETTE_SELECTOR_REGEX};
use crate::utils::bounded_float::BoundedFloat;

/// Parses a theme from a reader
pub fn from_reader<R: Read>(reader: R) -> Result<Theme, io::Error> {
    let json: ThemeJson = serde_json::from_reader(reader)?;
    Ok(from_theme_json(json))
}

pub fn from_str(reader: &str) -> Result<Theme, io::Error> {
    let json: ThemeJson = serde_json::from_str(reader)?;
    Ok(from_theme_json(json))
}

fn adjust_color(color: Color, theme: &Theme) -> Color {
    match color {
        Color::Var { var, fallback } if PALETTE_SELECTOR_REGEX.is_match(&var) => {
            let captures = PALETTE_SELECTOR_REGEX
                .captures(&var)
                .expect("must pass here");
            let palette = &captures["palette"];
            let selector = &captures["selector"];

            Color::Var {
                var: theme.palette_var(palette, selector),
                fallback,
            }
        }
        color => color,
    }
}
fn from_theme_json(json: ThemeJson) -> Theme {
    println!("json = {json:#?}");
    let mut theme = json
        .prefix
        .map(|prefix| Theme::with_prefix(prefix))
        .unwrap_or_else(Theme::new);
    for (palette_name, def) in json.palettes {
        let mut palette = Palette::new();
        if let Some(GradientJson {
            points: gradient,
            mode,
        }) = def.gradient
        {
            use GradientMode::*;
            let gradient: Gradient = match mode {
                None => gradient,
                Some(Hsl) => gradient
                    .into_iter()
                    .map(|(pt, c)| (pt, c.to_hsla_color().expect("could not convert to hsla")))
                    .collect(),
                Some(Rgb) => gradient
                    .into_iter()
                    .map(|(pt, c)| (pt, c.to_rgba_color().expect("could not convert to rgba")))
                    .collect(),
            };
            for i in 0..=10 {
                let as_float = BoundedFloat::new(i as f32 / 10.0).expect("must be valid");
                palette.insert_constant(&format!("{:03}", i * 10), gradient.get(as_float));
            }
        }
        if let Some(selectors) = def.selectors {
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
enum GradientMode {
    Hsl,
    Rgb,
}

#[derive(Debug, Deserialize)]
struct PaletteJson {
    gradient: Option<GradientJson>,
    selectors: Option<IndexMap<String, SelectorJson>>,
}

#[derive(Debug, Deserialize)]
struct GradientJson {
    points: Gradient,
    mode: Option<GradientMode>,
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
