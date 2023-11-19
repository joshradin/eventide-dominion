//! Surfaces provide work


use stylist::ast::Sheet;
use yew::{Children, function_component, Html, html, Properties};
use crate::theme::context::use_theme;
use crate::theme::sx::Sx;

#[derive(Default, Debug, Clone, PartialEq, Properties)]
pub struct BoxProps {
    #[prop_or_default]
    pub sx: Sx,
    #[prop_or_default]
    pub children: Children
}

#[function_component]
pub fn Box(props: &BoxProps) -> Html {
    let theme = use_theme();
    let sx = &props.sx;

    let sheet = Sheet::new();

    html! {
        <div class={stylist::css!(
            bgcolor: "red";
        )}>
            { for props.children.clone()}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use yew::ServerRenderer;
    use super::*;

    #[tokio::test]
    async fn styled_box() {
        let renderer = ServerRenderer::<Box>::with_props(|| { BoxProps::default()});
        let s = renderer.render().await;
        println!("{s}");
    }
}