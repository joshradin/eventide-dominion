//! The typography system allows for complex writing instrumentation


use yew::{Children, function_component, Html, html, Properties, ContextProvider, html_nested};
use crate::Sx;
use crate::system::Box;

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct TypographyProps {
    #[prop_or_else(|| "".to_string())]
    pub component: String,
    #[prop_or_default]
    pub sx: Sx,
    #[prop_or_default]
    pub children: Children
}

#[function_component]
pub fn Typography(props: &TypographyProps) -> Html {
    let context = yew::use_context::<TypographyContext>();
    let TypographyProps { component, sx, children, ..} = props;

    let component = yew::use_memo((component.clone(), context.clone()), |(comp, ctx)|
        if comp.is_empty() {
            return if ctx.is_some() {
                "span".to_string()
            } else {
                "p".to_string()
            }
        } else {
            comp.clone()
        }
    );

    let inner = html_nested! {
        <Box class={"typography"} sx={sx.clone()} component={(*component).clone()}>
            { for props.children.iter() }
        </Box>
    };

    match context {
        Some(context) => {
            html! {
                {inner}
            }
        },
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
struct TypographyContext { }