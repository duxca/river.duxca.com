pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/v20.0/dialog/oauth", axum::routing::get(authorize))
        .route("/v20.0/oauth/access_token", axum::routing::post(token))
        .route("/v20.0/me", axum::routing::get(user))
}

#[derive(Debug, serde::Deserialize)]
struct AuthorizeQuery {
    redirect_uri: String,
    state: Option<String>,
}

#[tracing::instrument(level = "trace")]
async fn authorize(
    axum::extract::Query(query): axum::extract::Query<AuthorizeQuery>,
) -> Result<impl axum::response::IntoResponse, crate::web::Ise> {
    let mut redirect_uri = reqwest::Url::parse(&query.redirect_uri)?;
    {
        let mut query_pairs = redirect_uri.query_pairs_mut();
        query_pairs.append_pair("code", "fake-facebook-code");
        if let Some(state) = query.state {
            query_pairs.append_pair("state", &state);
        }
    }

    Ok(axum::response::Redirect::to(redirect_uri.as_str()))
}

#[derive(Debug, serde::Serialize)]
struct TokenResponse {
    access_token: &'static str,
    token_type: &'static str,
}

#[tracing::instrument(level = "trace")]
async fn token() -> impl axum::response::IntoResponse {
    axum::Json(TokenResponse {
        access_token: "fake-facebook-access-token",
        token_type: "bearer",
    })
}

#[derive(Debug, serde::Serialize)]
struct UserResponse {
    id: &'static str,
    name: &'static str,
}

#[tracing::instrument(level = "trace", skip(headers))]
async fn user(headers: axum::http::HeaderMap) -> impl axum::response::IntoResponse {
    use axum::response::IntoResponse;

    let authorized = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value == "Bearer fake-facebook-access-token");

    if !authorized {
        return (
            axum::http::StatusCode::UNAUTHORIZED,
            axum::Json(serde_json::json!({
                "error": {
                    "message": "Invalid OAuth access token."
                }
            })),
        )
            .into_response();
    }

    axum::Json(UserResponse {
        id: "1",
        name: "fake-facebook-user",
    })
    .into_response()
}
