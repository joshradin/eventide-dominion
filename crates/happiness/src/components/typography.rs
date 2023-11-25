//! The typography system allows for complex writing instrumentation

use std::collections::HashMap;
use std::fmt::Display;

use yew::{Children, classes, ContextProvider, function_component, html, Html, html_nested, Properties};
use yew::html::IntoPropValue;

use crate::style::Size;
use crate::Sx;
use crate::system::StylingBox;
use crate::theme::typography::TypographyLevel;

pub type TypographyLevelMapping = HashMap<TypographyLevel, String>;

fn default_level_mapping() -> TypographyLevelMapping {
    use Size::*;
    use TypographyLevel::*;
    [
        (H1, "h1"),
        (H2, "h2"),
        (H3, "h3"),
        (H4, "h4"),
        (Title { size: Xs }, "p"),
        (Title { size: Sm }, "p"),
        (Title { size: Md }, "p"),
        (Title { size: Lg }, "p"),
        (Title { size: Xl }, "p"),
        (Body { size: Xs }, "span"),
        (Body { size: Sm }, "p"),
        (Body { size: Md }, "p"),
        (Body { size: Lg }, "p"),
        (Body { size: Xl }, "p"),
    ]
    .into_iter()
    .map(|(k, v)| (k, v.to_string()))
    .collect()
}

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct TypographyProps {
    #[prop_or_else(|| "".to_string())]
    pub component: String,
    #[prop_or_default]
    pub sx: Sx,
    #[prop_or_default]
    pub level: TypographyLevel,
    #[prop_or_else(default_level_mapping)]
    pub mapping: TypographyLevelMapping,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn Typography(props: &TypographyProps) -> Html {
    let context = yew::use_context::<TypographyContext>();
    let TypographyProps {
        component,
        sx,
        children,
        level,
        mapping,
        ..
    } = props;

    let component = yew::use_memo(
        (
            component.clone(),
            context.clone(),
            level.clone(),
            mapping.clone(),
        ),
        |(comp, ctx, level, mapping)| {
            if comp.is_empty() {
                let default = &mapping[level];

                return if ctx.is_some() {
                    if default == "p" {
                        "span".to_string()
                    } else {
                        default.clone()
                    }
                } else {
                    default.clone()
                };
            } else {
                comp.clone()
            }
        },
    );

    let classes = classes!("typography", level.to_string());
    let inner = html_nested! {
        <StylingBox class={classes} sx={sx.clone()} component={(*component).clone()}>
            { for props.children.iter() }
        </StylingBox>
    };

    match context {
        Some(context) => {
            html! {
                {inner}
            }
        }
        None => {
            html! {
                <ContextProvider<TypographyContext> context={TypographyContext::default()}>
                    {inner}
                </ContextProvider<TypographyContext>>
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
struct TypographyContext {}
