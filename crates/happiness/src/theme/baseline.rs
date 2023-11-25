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
            let mut selector = palette.select(selector_name, mode).unwrap().clone();
            if let Ok(adjusted) = selector.to_rgba_color() {
                selector = adjusted;
            }
            to_merge.insert(
                theme.palette_var(palette_name, selector_name),
                SxValue::Color(selector),
            )
        }
        emit = emit.merge(sx! {
            "html": to_merge
        })
    }

    emit.merge(sx! {
        ":root": sx! {
            "color": "text.primary",
            "bgcolor": "background.level1",
        },
        (format!(".{}-system", theme.prefix)): {
            "[color=success]": {
                "[variant=outlined]": {
                    "borderWidth": "3px",
                    "borderColor": "success.outlinedBorder",
                    "borderStyle": "solid",
                    "padding": "3px",
                    "color": "success.outlinedColor",
                    "[disabled]": {
                        "borderColor": "success.outlinedDisabledBorder",
                        "color": "success.outlinedDisabledColor",
                    }
                }
            }
        }
    })
    .merge(sx! {
        "body": {
            "background-color": "background.body",
            "margin": "0px"
        },
    })
}
