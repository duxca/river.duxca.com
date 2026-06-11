#[allow(clippy::single_component_path_imports)]
#[allow(unused_imports)]
use app;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::App;

    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
