//! Contains system components. Not meant for general use

use crate::theme::sx::Sx;
use yew::html::Children;
use crate::theme::hooks::use_sx;
use yew::{function_component, Html, html, Properties};



#[derive(Default, Debug, Clone, PartialEq, Properties)]
pub struct BoxProps {
    #[prop_or_default]
    pub sx: Sx,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn Box(props: &BoxProps) -> Html {
    let sx = use_sx(props.sx.clone());

    html! {
        <div class={sx}>
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
        let renderer = ServerRenderer::<Box>::new();
        let s = renderer.render().await;
        println!("{s}");
    }
}
