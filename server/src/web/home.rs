/// GET /
#[tracing::instrument(level = "trace", skip(auth_session, st))]
pub async fn home(
    auth_session: axum_login::AuthSession<crate::web::login::Backend>,
    axum::extract::State(ref st): axum::extract::State<crate::web::State>,
    req: axum::http::Request<axum::body::Body>,
) -> Result<impl axum::response::IntoResponse, crate::web::Ise> {
    use axum::response::IntoResponse;
    use leptos::prelude::*;

    let user = auth_session.user;
    let mut account = app::AccountContext::default();
    let auths = if let Some(user) = user.as_ref() {
        let mut conn = st.db.acquire().await?;
        account.delete_preview =
            Some(db::user::get_user_delete_preview(&mut conn, user.user_id).await?);
        db::user::get_user_auths(&mut conn, user.user_id).await?
    } else {
        vec![]
    };

    let providers = app::AuthProviders::from_auths(&auths);
    let handler = leptos_axum::render_app_to_stream_with_context(
        || {},
        move || {
            view! {
                <app::HomePage
                    user=user.clone()
                    providers=providers.clone()
                    account=account.clone()
                />
            }
        },
    );

    Ok(handler(req).await.into_response())
}
