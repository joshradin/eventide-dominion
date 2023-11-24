use crate::theme::sx::Sx;
use crate::theme::Theme;
use crate::{components::system::Box, sx, use_sx};
use yew::{function_component, html, Children, Html, Properties};

#[derive(Default, Debug, Clone, PartialEq, Properties)]
pub struct SheetProps {
    #[prop_or_default]
    pub sx: Sx,
    #[prop_or_else(|| "".to_string())]
    pub variant: String,
    #[prop_or_else(|| "".to_string())]
    pub color: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn Sheet(props: &SheetProps) -> Html {
    let sx = props.sx.clone().merge(sx! {
        background: SxValue::var("sheet", "background-color", None)
    });

    html! {
        <Box {sx} class={yew::classes!("sheet")}>
            {for props.children.clone()}
        </Box>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use yew::html::IntoPropValue;
    use yew::{html, ServerRenderer};

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
