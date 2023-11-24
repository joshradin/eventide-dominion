use log::info;
use wasm_bindgen_test::{console_log, wasm_bindgen_test};
use yew::{function_component, html, use_callback, Callback, Html, Renderer};

use happiness::theme::hooks::use_mode;
use happiness::theme::theme_mode::ThemeMode;
use happiness::theme::Theme;
use happiness::{surfaces::Sheet, sx, CssBaseline, ThemeProvider};

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[function_component]
fn App() -> Html {
    let theme = Theme::default();
    let (mode, set_mode): (ThemeMode, Callback<ThemeMode>) = use_mode();

    let onclick = {
        use_callback(
            mode.clone(),
            move |_: yew::events::MouseEvent, mode| match mode {
                ThemeMode::Light => set_mode.emit(ThemeMode::Dark),
                ThemeMode::Dark | ThemeMode::System => set_mode.emit(ThemeMode::Light),
            },
        )
    };

    html! {
       <ThemeProvider theme={theme}>
            <CssBaseline />
            <Main />
        </ThemeProvider>
    }
}

#[function_component]
fn Main() -> Html {
    let theme = Theme::default();
    let (mode, set_mode): (ThemeMode, Callback<ThemeMode>) = use_mode();

    let onclick = {
        use_callback(
            mode.clone(),
            move |_: yew::events::MouseEvent, mode| match mode {
                ThemeMode::Light => set_mode.emit(ThemeMode::Dark),
                ThemeMode::Dark | ThemeMode::System => set_mode.emit(ThemeMode::Light),
            },
        )
    };

    html! {
        <Sheet sx={ sx!{
                    "p": "15px",
                    "bgcolor": "background.level1",
                 }}>
                <Sheet
                    sx={sx!{
                        "backgroundColor": "background.level2",
                        "padding": "10px"
                    }}
                >
                {"Hello, World!"}
                    <button {onclick}>{format!("{:?}", mode)}</button>
                </Sheet>
            </Sheet>
    }
}

#[wasm_bindgen_test]
async fn create_css() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    info!("starting test");
    let handle = Renderer::<App>::new().render();
}
