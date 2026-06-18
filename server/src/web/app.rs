#[tracing::instrument(level = "trace", skip(auth_session, st, req))]
pub async fn app_shell(
    auth_session: axum_login::AuthSession<crate::web::login::Backend>,
    axum::extract::State(ref st): axum::extract::State<crate::web::State>,
    req: axum::http::Request<axum::body::Body>,
) -> Result<impl axum::response::IntoResponse, crate::web::Ise> {
    use axum::response::IntoResponse;
    use leptos::prelude::*;

    let options = st.leptos_options.clone();
    let user = auth_session.user;
    let db = st.db.clone();
    let handler = leptos_axum::render_app_to_stream_with_context(
        move || {
            if let Some(user) = user.clone() {
                provide_context(shared_api::ServerApiContext {
                    db: db.clone(),
                    user,
                });
            }
        },
        move || app::shell(options.clone()),
    );

    Ok(handler(req).await.into_response())
}
