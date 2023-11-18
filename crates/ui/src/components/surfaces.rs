//! Contains surfaces


use yew::{Children, classes, function_component, Html, html, Properties};

#[derive(Debug, PartialEq, Properties)]
pub struct SurfaceProps {
    pub children: Children,
    #[prop_or_default]
    pub outlined: bool
}

#[function_component]
pub fn Sheet(props: &SurfaceProps) -> Html {
    html! {
        <div class={classes!("box")}>
            {for props.children.clone()}
        </div>
    }
}

#[function_component]
pub fn Card(props: &SurfaceProps) -> Html {
    let mut classes = classes!("card");
    if props.outlined {
        classes.push("outlined");
    }
    html! {
        <div class={classes}>
            {for props.children.clone()}
        </div>
    }
}