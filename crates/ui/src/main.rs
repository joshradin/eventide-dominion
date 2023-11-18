use uuid::Uuid;
use yew::{Callback, classes, function_component, html, Html, use_state};
use yew::platform::spawn_local;
use ui::components::layout::Stack;

use ui::Request;
use ui::components::surfaces::*;
use happiness::surfaces::Box;
#[function_component]
fn App() -> Html {
    let uuid = use_state(|| Option::<Uuid>::None);
    let handle_click = {
        let uuid = uuid.clone();
        Callback::from(move |_event| {
            let uuid = uuid.clone();
            spawn_local(async move {
                let result = Request::get("/random_uuid")
                    .send()
                    .await
                    .unwrap()
                    .json::<Uuid>()
                    .await
                    .unwrap();
                uuid.set(result.into());
            });
        })
    };

    html! {
        <Box>
            <Card outlined=true>
                <Stack>
                    if let Some(uuid) = *uuid {
                        <p>{"Generated Uuid: "}{uuid}</p>
                    }
                    <Sheet>
                        <button onclick={handle_click}>{"Generate new Id!"}</button>
                    </Sheet>
                </Stack>
            </Card>
        </Box>

    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<App>::new().render();
}
