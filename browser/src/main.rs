mod api;
mod components;
mod hooks;

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

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
