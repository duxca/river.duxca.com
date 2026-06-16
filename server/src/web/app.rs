#[tracing::instrument(level = "trace", skip(auth_session, st, req))]
pub async fn app_shell(
    auth_session: axum_login::AuthSession<crate::web::login::Backend>,
    axum::extract::State(ref st): axum::extract::State<crate::web::State>,
    req: axum::http::Request<axum::body::Body>,
) -> Result<impl axum::response::IntoResponse, crate::web::Ise> {
    use axum::response::IntoResponse;
    use leptos::prelude::provide_context;

    let options = st.leptos_options.clone();
    let ctx = auth_session.user.map(|user| shared_api::ServerApiContext {
        db: st.db.clone(),
        user,
    });
    let handler = leptos_axum::render_app_async_with_context(
        move || {
            if let Some(ctx) = ctx.clone() {
                provide_context(ctx);
            }
        },
        move || app::shell(options.clone()),
    );

    Ok(handler(req).await.into_response())
}
