use wasm_bindgen_test::{console_log, wasm_bindgen_test};
use yew::{function_component, html, Html, Renderer};

use happiness::{CssBaseline, surfaces::Sheet, ThemeProvider, sx};
use happiness::theme::Theme;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[function_component]
fn App() -> Html {
    let theme = Theme::default();

    html! {
       <ThemeProvider theme={theme}>
            <CssBaseline />
            <Sheet>
                <Sheet
                    sx={sx!{
                        "backgroundColor": "common.white",
                        "margin": "10px"
                    }}
                >
                {"Hello, World!"}
                </Sheet>
            </Sheet>
        </ThemeProvider>
    }
}

#[wasm_bindgen_test]
async fn create_css() {
    console_log!("starting test app");

    let handle = Renderer::<App>::new().render();
    console_log!("handle: {:#?}", handle);
}
