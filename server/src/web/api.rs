#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
pub enum UnAuthRequest {}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
pub enum UnAuthResponse {}

/// POST /api
#[tracing::instrument(level = "trace", skip(auth_session, st))]
pub async fn api(
    auth_session: axum_login::AuthSession<crate::web::login::Backend>,
    axum::extract::State(ref st): axum::extract::State<crate::web::State>,
    axum::extract::Json(json): axum::extract::Json<serde_json::Value>,
) -> Result<impl axum::response::IntoResponse, crate::web::Ise> {
    use axum::response::IntoResponse;
    let user = auth_session.user;
    let Some(user) = user else {
        return Ok((axum::http::StatusCode::UNAUTHORIZED, "401").into_response());
    };
    let Ok(req) = serde_json::from_value::<model::api::Request>(json) else {
        return Ok((axum::http::StatusCode::BAD_REQUEST, "invalid request").into_response());
    };
    let res = service::handler(&st.db, &user, req).await?;
    let json = serde_json::to_value(res)?;
    Ok(axum::response::Json::from(json).into_response())
}
