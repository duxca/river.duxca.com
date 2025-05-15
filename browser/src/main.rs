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
    yew::Renderer::<App>::new().render();
}
