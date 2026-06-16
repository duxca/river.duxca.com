#[allow(clippy::single_component_path_imports)]
#[allow(unused_imports)]
use app;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::App;

    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    let use_islands = leptos::web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.query_selector("leptos-island").ok().flatten())
        .is_some();

    if use_islands {
        leptos::mount::hydrate_islands();
    } else {
        leptos::mount::hydrate_body(App);
    }
}
