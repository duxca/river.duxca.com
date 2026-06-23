mod pages;

use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

pub use pages::{
    AccountContext, AdminPage, AdminPageData, AuthProviders, HomePage, HomePageData, LoginPage,
    MapPage,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="ja">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <MetaTags/>
                <link
                    rel="stylesheet"
                    href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css"
                    integrity="sha256-p4NxAoJBhIIN+hmNHrzRCf9tD/miZyoHS5obTRR9BMY="
                    crossorigin=""
                />
                <script
                    src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js"
                    integrity="sha256-20nQCchB9co0qIjJZRGuk2/Z9VM+kNiyxNV1lvTlZBo="
                    crossorigin=""
                ></script>
                <script src="/register-sw.js"></script>
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
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/app/pkg/frontend.css"/>
        <Title text="river.duxca.com Leptos map"/>
        <Router>
            <Routes fallback=|| view! { <p>"ページが見つかりません"</p> }>
                <Route path=StaticSegment("") view=HomePage/>
                <Route path=StaticSegment("login") view=LoginPage/>
                <Route path=StaticSegment("app") view=MapPage/>
            </Routes>
        </Router>
    }
}
