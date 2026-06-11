#[tracing::instrument(level = "trace", skip(st, req))]
pub async fn app_shell(
    axum::extract::State(ref st): axum::extract::State<crate::web::State>,
    req: axum::http::Request<axum::body::Body>,
) -> Result<impl axum::response::IntoResponse, crate::web::Ise> {
    use axum::response::IntoResponse;

    let options = st.leptos_options.clone();
    let handler = leptos_axum::render_app_to_stream(move || app::shell(options.clone()));

    Ok(handler(req).await.into_response())
}
