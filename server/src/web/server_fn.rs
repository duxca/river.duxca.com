/// POST /api/{*fn_name}
#[tracing::instrument(level = "trace", skip(auth_session, st, req))]
pub async fn server_fn(
    auth_session: axum_login::AuthSession<crate::web::login::Backend>,
    axum::extract::State(ref st): axum::extract::State<crate::web::State>,
    req: axum::http::Request<axum::body::Body>,
) -> Result<impl axum::response::IntoResponse, crate::web::Ise> {
    use axum::response::IntoResponse;
    use leptos::prelude::provide_context;

    let Some(user) = auth_session.user else {
        return Ok((axum::http::StatusCode::UNAUTHORIZED, "401").into_response());
    };
    let ctx = shared_api::ServerApiContext {
        db: st.db.clone(),
        user,
    };
    let res =
        leptos_axum::handle_server_fns_with_context(move || provide_context(ctx.clone()), req)
            .await;
    Ok(res.into_response())
}
