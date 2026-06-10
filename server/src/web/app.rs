#[tracing::instrument(level = "trace", skip(st, req))]
pub async fn app_shell(
    axum::extract::State(ref st): axum::extract::State<crate::web::State>,
    req: axum::http::Request<axum::body::Body>,
) -> Result<impl axum::response::IntoResponse, crate::web::Ise> {
    use axum::response::IntoResponse;
    use leptos::prelude::*;

    let options = st.leptos_options.clone();
    let css_path = format!("/app{}", options.css_path());
    let handler = leptos_axum::render_app_to_stream(move || {
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
                    <link rel="stylesheet" href=css_path.clone()/>
                    <AutoReload options=options.clone()/>
                    <HydrationScripts options=options.clone() root="/app"/>
                </head>
                <body></body>
            </html>
        }
    });

    Ok(handler(req).await.into_response())
}
