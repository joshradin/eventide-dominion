//! Contains system components. Not meant for general use

use web_sys::HtmlElement;
use crate::sx;
use crate::theme::hooks::use_sx;
use crate::theme::sx::Sx;
use yew::html::Children;
use yew::{classes, function_component, html, Classes, Html, Properties, use_effect_with, NodeRef};
use crate::style::{Color, Variant};

#[derive(Default, Debug, Clone, PartialEq, Properties)]
pub struct BoxProps {
    #[prop_or_default]
    pub sx: Sx,
    #[prop_or_default]
    pub variant: Option<Variant>,
    #[prop_or_default]
    pub color: Option<Color>,
    #[prop_or_else(|| "div".to_string())]
    pub component: String,
    #[prop_or_else(|| classes!("box"))]
    pub class: Classes,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn Box(props: &BoxProps) -> Html {
    let sx = use_sx(props.sx.clone());
    let mut classes = classes!(sx, "box");
    classes.extend(props.class.clone());

    let html_ref = yew::use_node_ref();
    {
        let html_ref = html_ref.clone();
        use_effect_with((props.variant, props.color, props.disabled, html_ref), |(variant, color, disabled, node)| {
            info!("setting attributes for color and variant for node: {node:?}");
            let element: HtmlElement = node.cast::<HtmlElement>().expect("should be an html element");
            if let Some(variant) = variant {
                element.set_attribute("variant", &variant.to_string()).expect("could not set variant attribute");
            }
            if let Some(color) = color {
                element.set_attribute("color", &color.to_string()).expect("could not set color attribute");
            }
            if *disabled {
                element.set_attribute("disabled", "").expect("could not set color attribute");
            } else {
                let _ =element.remove_attribute("disabled");
            }
        });
    }

    html! {
        <@{props.component.clone()} class={classes} ref={html_ref}>
            { for props.children.clone() }
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
