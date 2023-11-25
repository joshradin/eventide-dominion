use log::info;
use wasm_bindgen_test::wasm_bindgen_test;
use yew::{Callback, function_component, html, Html, Renderer, use_callback};

use happiness::{CssBaseline, surfaces::Sheet, ThemeProvider, typography::Typography};
use happiness::theme::hooks::use_mode;
use happiness::theme::Theme;
use happiness::theme::theme_mode::ThemeMode;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[function_component]
fn App() -> Html {
    let theme = yew::functional::use_mut_ref(|| Theme::default());

    html! {
       <ThemeProvider theme={theme.borrow().clone()}>
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
        <Sheet variant={"outlined"} color={"success"}>
            <Typography>
                {"Hello, "}<Typography>{"World!"}</Typography>
            </Typography>
            <button {onclick}>{format!("{:?}", mode)}</button>

        </Sheet>
    }
}

#[wasm_bindgen_test]
async fn create_css() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    info!("starting test");
    let handle = Renderer::<App>::new().render();
}
