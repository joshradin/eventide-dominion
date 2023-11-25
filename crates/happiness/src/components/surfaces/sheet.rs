use yew::{Children, function_component, html, Html, Properties};

use crate::{components::system::Box, sx, theme::sx::SxValue};
use crate::style::{Color, Variant};
use crate::theme::sx::Sx;

#[derive(Default, Debug, Clone, PartialEq, Properties)]
pub struct SheetProps {
    #[prop_or_default]
    pub sx: Sx,
    #[prop_or_default]
    pub variant: Variant,
    #[prop_or_default]
    pub color: Color,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn Sheet(props: &SheetProps) -> Html {
    let sx = props.sx.clone().merge(sx! {
        "bgcolor": SxValue::var("sheet", "background-color", None),
    });
    let SheetProps { color, variant, .. } = props;

    html! {
        <Box {sx} class={yew::classes!("sheet")} {color} {variant}>
            {for props.children.clone()}
        </Box>
    }
}

#[cfg(test)]
mod tests {
    use yew::{html, ServerRenderer};

    use super::*;

    #[tokio::test]
    async fn render_sheet() {
        #[function_component]
        fn Test() -> Html {
            html! {
                <Sheet>

                </Sheet>
            }
        }

        let rendered = ServerRenderer::<Test>::new().render().await;
        println!("{rendered:?}")
    }
}
