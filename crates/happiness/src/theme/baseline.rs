use crate::{Sx, sx};

/// Creates the base style sheet for happiness
pub fn baseline() -> Sx {
    let mut emit = sx!();

    emit
        .merge(sx! {
            ":root": sx! {
            }
        })
        .merge(
        sx! {
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
        }
    )
}