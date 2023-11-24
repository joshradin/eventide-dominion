use stylist::Style;
use yew::{hook, use_memo};

use crate::theme::hooks::{use_style_manager, use_theme};
use crate::theme::sx::{Sx, SxRef};
use crate::theme::theme_mode::ThemeMode;
use crate::theme::Theme;

#[hook]
pub fn use_sx<Source>(source: Source) -> SxRef
where
    Source: Into<Sx>,
{
    let ctx = use_theme();
    let manager = use_style_manager();

    let sx = source.into();
    let css = use_memo((sx, ctx), |(sx, ctx)| {
        let theme: &Theme = &*ctx;
        sx.clone().to_css(&ThemeMode::System, theme)
    });

    let style = Style::new_with_manager((*css).clone(), &*manager).expect("could not create style");
    SxRef::new(style)
}
