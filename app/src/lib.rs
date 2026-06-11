mod pages;

use leptos::prelude::*;
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

pub use pages::{AccountContext, AuthProviders, HomePage, LoginPage, MapPage};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    let css_path = format!("/app{}", options.css_path());
    view! {
        <!DOCTYPE html>
        <html lang="ja">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <title>"river.duxca.com Leptos map"</title>
                <link
                    rel="stylesheet"
                    href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css"
                    integrity="sha256-p4NxAoJBhIINfQfb3HYJZqd6ZewBskNiyxNV1lvTlZBo="
                    crossorigin=""
                />
                <script
                    src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js"
                    integrity="sha256-20nQCchB9co0qIjJZRGuk2/Z9VM+kNiyxNV1lvTlZBo="
                    crossorigin=""
                ></script>
                <link rel="stylesheet" href=css_path/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options=options.clone() root="/app"/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router base="/app">
            <Routes fallback=|| view! { <p>"ページが見つかりません"</p> }>
                <Route path=StaticSegment("") view=MapPage/>
            </Routes>
        </Router>
    }
}
