use std::collections::HashMap;
use std::io;
use std::io::Read;
use serde::Deserialize;
use crate::theme::palette::Palette;
use crate::theme::{Color, Theme};

pub fn from_reader<R : Read>(reader: R) -> Result<Theme, io::Error> {
    let json: ThemeJson = serde_json::from_reader(reader)?;
    Ok(from_theme_json(json))
}

pub fn from_str(reader: &str) -> Result<Theme, io::Error> {
    let json: ThemeJson = serde_json::from_str(reader)?;
    Ok(from_theme_json(json))
}

fn from_theme_json(json: ThemeJson) -> Theme {
    let mut theme = Theme::new();
    for (palette_name, def) in json.palettes {
        let mut palette = Palette::new();
        for (selector, color) in def.selectors {
            match color {
                ColorJson::Const(c) => {
                    palette.insert_constant(&selector, Color::named(c));
                }
                ColorJson::DarkLight { dark, light } => {
                    palette.insert_by_mode(&selector, Color::named(dark), Color::named(light));
                }
            }
        }
        theme.insert_palette(palette_name, palette);
    }
    theme
}

#[derive(Debug, Deserialize)]
struct ThemeJson {
    palettes: HashMap<String, PaletteJson>
}

#[derive(Debug, Deserialize)]
struct PaletteJson {
    #[serde(flatten)]
    selectors: HashMap<String, ColorJson>
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ColorJson {
    Const(String),
    DarkLight {
        dark: String,
        light: String
    }
}

#[cfg(test)]
mod tests {
    use crate::theme::serde::{from_reader, from_str};

    #[test]
    fn parse_theme_json() {
        let json = include_str!("./theme.json");
        let parsed = from_str(json).expect("could not parse");

        println!("parsed: {:#?}", parsed);
    }
}