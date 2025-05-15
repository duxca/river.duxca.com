mod api;
mod components;
mod hooks;

use yew::prelude::*;
use components::home::Home;

#[function_component(App)]
#[allow(clippy::redundant_closure)]
fn app() -> HtmlResult {
    let html = html! {
        <Suspense>
            <Home />
        </Suspense>
    };
    Ok(html)
}

fn main() {
    yew::Renderer::<App>::new().render();
}
