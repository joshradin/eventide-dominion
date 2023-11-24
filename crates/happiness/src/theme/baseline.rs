use crate::theme::sx::SxValue;
use crate::theme::theme_mode::ThemeMode;
use crate::theme::Theme;
use crate::{sx, Sx};

/// Creates the base style sheet for happiness
pub fn baseline(theme: &Theme, mode: &ThemeMode) -> Sx {
    let mut emit = sx!();

    for (palette_name, palette) in theme.palettes() {
        let mut to_merge = sx!();
        for selector_name in palette.selectors() {
            let selector = palette.select(selector_name, mode).unwrap();
            to_merge.insert(
                theme.palette_var(palette_name, selector_name),
                SxValue::Color(selector.clone()),
            )
        }
        emit = emit.merge(sx! {
            ":root": to_merge
        })
    }

    emit.merge(sx! {
        ":root": sx! {
            "color": "text.primary"
        }
    })
    .merge(sx! {
        "body": {
            "background-color": "background.body",
            "margin": "0px"
        },
        ".sheet": {
            "--happy-sheet-background-color": "background.level1",
        }
    })
}
