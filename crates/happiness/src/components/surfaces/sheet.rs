use yew::{Children, function_component, Html, html, Properties};
use crate::theme::sx::Sx;
use crate::{use_sx, sx, components::system::Box};

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
    let sx = props.sx.clone().merge(sx!{
        background: "background.body"
    });

    html! {
        <Box {sx}>
            {for props.children.clone()}
        </Box>
    }
}


#[cfg(test)]
mod tests {
    use yew::{html, ServerRenderer};
    use yew::html::IntoPropValue;
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