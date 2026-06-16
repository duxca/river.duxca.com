#[tracing::instrument(level = "trace", skip(auth_session, st, req))]
pub async fn app_shell(
    auth_session: axum_login::AuthSession<crate::web::login::Backend>,
    axum::extract::State(ref st): axum::extract::State<crate::web::State>,
    req: axum::http::Request<axum::body::Body>,
) -> Result<impl axum::response::IntoResponse, crate::web::Ise> {
    use axum::response::IntoResponse;
    use leptos::prelude::provide_context;

    let path = req.uri().path().to_owned();
    let user = auth_session.user;
    if path == "/login" && user.is_some() {
        return Ok(axum::response::Redirect::to("/").into_response());
    }

    let mut account = app::AccountContext::default();
    let auths = if let Some(user) = user.as_ref() {
        let mut conn = st.db.acquire().await?;
        account.delete_preview =
            Some(db::user::get_user_delete_preview(&mut conn, user.user_id).await?);
        db::user::get_user_auths(&mut conn, user.user_id).await?
    } else {
        vec![]
    };
    let home = app::HomePageData {
        user,
        providers: app::AuthProviders::from_auths(&auths),
        account,
    };
    let options = st.leptos_options.clone();
    let handler = leptos_axum::render_app_to_stream_with_context(
        move || {
            provide_context(home.clone());
        },
        move || app::shell(options.clone()),
    );

    Ok(handler(req).await.into_response())
}
