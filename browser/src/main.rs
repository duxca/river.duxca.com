mod api;
mod components;
mod hooks;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[function_component(App)]
#[allow(clippy::redundant_closure)]
fn app() -> HtmlResult {
    let html = html! {
        <Suspense>
            <crate::components::login::Login />
        </Suspense>
    };
    Ok(html)
}

#[wasm_bindgen(main)]
async fn main() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    gloo::timers::future::sleep(std::time::Duration::from_millis(1)).await;
    yew::Renderer::<App>::new().render();
    Ok(())
}
