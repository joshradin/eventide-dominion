//! Contains system components. Not meant for general use

use crate::sx;
use crate::theme::hooks::use_sx;
use crate::theme::sx::Sx;
use yew::html::Children;
use yew::{classes, Classes, function_component, html, Html, Properties};

#[derive(Default, Debug, Clone, PartialEq, Properties)]
pub struct BoxProps {
    #[prop_or_default]
    pub sx: Sx,
    #[prop_or_else(|| "div".to_string())]
    pub component: String,
    #[prop_or_else(|| classes!("box"))]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn Box(props: &BoxProps) -> Html {
    let sx = use_sx(props.sx.clone());
    let mut classes = classes!(sx);
    classes.extend(props.class.clone());

    html! {
        <@{props.component.clone()} class={classes}>
            { for props.children.clone()}
        </@>
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
