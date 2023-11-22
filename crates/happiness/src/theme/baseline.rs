use crate::theme::{Theme, ThemeMode};
use crate::{sx, Sx};
use crate::theme::sx::SxValue;

/// Creates the base style sheet for happiness
pub fn baseline(theme: &Theme, mode: &ThemeMode) -> Sx {
    let mut emit = sx!();


    for (palette_name, palette) in theme.palettes() {
        let mut to_merge = sx!();
        for selector_name in palette.selectors() {
            let selector = palette.select(selector_name, mode).unwrap();
            to_merge.insert(theme.var_name(palette_name, selector_name), SxValue::Color(selector.clone()))
        }
        emit =emit.merge(sx! {
            ":root": to_merge
        })
    }


    emit.merge(sx! {
        ":root": sx! {
        }
    })
    .merge(sx! {
        "body": {
            "background-color": "background.body",
        },
        ".sheet": {
            "background-color": "background.level1",
            ".sheet": {
                "background-color": "background.level2",
                ".sheet": {
                    "background-color": "background.level3",
                }
            }
        }
    })
}
