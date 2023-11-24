use ui::components::layout::Stack;
use uuid::Uuid;
use yew::platform::spawn_local;
use yew::{classes, function_component, html, use_state, Callback, Html};

use happiness::{system::Box, CssBaseline, ThemeProvider};
use ui::components::surfaces::*;
use ui::Request;
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
        <ThemeProvider>
            <CssBaseline />
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
        </ThemeProvider>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<App>::new().render();
}
