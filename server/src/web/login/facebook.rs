use anyhow::Context;

const AUTH_URL: &str = "https://www.facebook.com/v20.0/dialog/oauth";
const TOKEN_URL: &str = "https://graph.facebook.com/v20.0/oauth/access_token";
const USER_URL: &str = "https://graph.facebook.com/v20.0/me";
const CSRF_STATE_KEY: &str = "oauth.csrf-state";
// https://developers.facebook.com/apps/?show_reminder=true&locale=ja_JP
const REDIRECT_PATH: &str = "/oauth/callback/facebook";

#[derive(Debug, serde::Deserialize)]
pub struct LoginForm {}

/// POST /login/facebook
#[tracing::instrument(level = "trace", skip(auth_session, session))]
pub async fn login(
    auth_session: axum_login::AuthSession<crate::web::login::Backend>,
    session: tower_sessions::Session,
    axum::Form(LoginForm {}): axum::Form<LoginForm>,
) -> Result<impl axum::response::IntoResponse, crate::web::Ise> {
    use axum::response::IntoResponse;
    use anyhow::Context;
    let auth_url = oauth2::AuthUrl::new(AUTH_URL.to_string()).unwrap();
    let token_url = oauth2::TokenUrl::new(TOKEN_URL.to_string()).unwrap();
    let client_id = auth_session.backend.settings.facebook_client_id.clone();
    let client_secret = auth_session.backend.settings.facebook_client_secret.clone();
    let redirect_url = oauth2::RedirectUrl::new(format!(
        "{}{}",
        auth_session.backend.settings.base_url, REDIRECT_PATH
    ))
    .unwrap();
    let client = oauth2::basic::BasicClient::new(client_id)
        .set_client_secret(client_secret)
        .set_auth_uri(auth_url)
        .set_token_uri(token_url)
        .set_redirect_uri(redirect_url);
    let (auth_url, csrf_state) = client.authorize_url(oauth2::CsrfToken::new_random).url();
    session
        .insert(CSRF_STATE_KEY, csrf_state.secret())
        .await
        .context("Failed to insert CSRF state into session")?;
    session
        .save()
        .await
        .context("Failed to save session after CSRF state insertion")?;
    Ok(axum::response::Redirect::to(auth_url.as_str()).into_response())
}

// OAuth2 の認可コードを受け取るためのクエリパラメータ
#[derive(Debug, Clone, serde::Deserialize)]
pub struct AuthzRequestQuery {
    pub code: oauth2::AuthorizationCode,
    pub state: oauth2::CsrfToken,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Credentials {
    pub auth_code: oauth2::AuthorizationCode,
    pub user: Option<model::user::User>,
}

/// GET /oauth/callback/facebook
#[tracing::instrument(level = "trace", skip(auth_session, session))]
pub async fn callback(
    mut auth_session: axum_login::AuthSession<crate::web::login::Backend>,
    session: tower_sessions::Session,
    axum::extract::Query(AuthzRequestQuery {
        code: auth_code,
        state: incomming_state,
    }): axum::extract::Query<AuthzRequestQuery>,
) -> Result<impl axum::response::IntoResponse, crate::web::Ise> {
    use axum::response::IntoResponse;
    // セッションがない場合はエラー
    let Some(saved_state) = session
        .get::<oauth2::CsrfToken>(CSRF_STATE_KEY)
        .await
        .context("Failed to get CSRF state from session")?
    else {
        log::error!("cannot find csrf state");
        return Ok((axum::http::StatusCode::BAD_REQUEST, "session expired").into_response());
    };
    // Ensure the CSRF state has not been tampered with.
    if incomming_state.secret() != saved_state.secret() {
        return Ok((
            axum::http::StatusCode::UNAUTHORIZED,
            "authentication failed",
        )
            .into_response());
    };
    let creds = super::Credentials::Facebook(Credentials {
        auth_code,
        // ログイン済みかどうか
        user: auth_session.user.clone(),
    });
    let Some(user) = auth_session.authenticate(creds).await? else {
        return Ok((
            axum::http::StatusCode::UNAUTHORIZED,
            "authentication failed",
        )
            .into_response());
    };
    auth_session.login(&user).await?;
    Ok(axum::response::Redirect::to("/").into_response())
}

#[tracing::instrument(level = "trace")]
pub async fn get_access_token(
    client_id: oauth2::ClientId,
    client_secret: oauth2::ClientSecret,
    auth_code: oauth2::AuthorizationCode,
    base_url: &str,
) -> Result<oauth2::AccessToken, anyhow::Error> {
    let auth_url = oauth2::AuthUrl::new(AUTH_URL.to_string()).unwrap();
    let token_url = oauth2::TokenUrl::new(TOKEN_URL.to_string()).unwrap();
    let redirect_url = oauth2::RedirectUrl::new(format!("{}{}", base_url, REDIRECT_PATH)).unwrap();
    let client = oauth2::basic::BasicClient::new(client_id)
        .set_client_secret(client_secret)
        .set_auth_uri(auth_url)
        .set_token_uri(token_url)
        .set_redirect_uri(redirect_url);
    // Process authorization code, expecting a token response back.
    let token_res = client
        .exchange_code(auth_code)
        .request_async(&reqwest::Client::new())
        .await
        .context("Failed to exchange authorization code for access token")?;
    use oauth2::TokenResponse;
    let access_token = token_res.access_token().clone();
    Ok(access_token)
}

// Use access token to request user info.
#[derive(Debug, serde::Deserialize)]
pub struct UserInfo {
    // legokichi
    name: String,
    // fb unique id as numeric string
    id: String,
}

#[tracing::instrument(level = "trace")]
pub async fn get_me(access_token: &oauth2::AccessToken) -> Result<UserInfo, anyhow::Error> {
    let access_token = access_token.secret().as_str();
    let res = reqwest::Client::new()
        .get(format!(
            "{USER_URL}?fields=id,name&access_token={access_token}"
        ))
        .header(
            axum::http::header::AUTHORIZATION.as_str(),
            format!("Bearer {access_token}"),
        )
        .header(axum::http::header::USER_AGENT.as_str(), "axum-login")
        .send()
        .await
        .context("Failed to send request to Facebook API")?;
    let user_info = res.text().await.context("Failed to read Facebook API response")?;
    log::debug!("{}", user_info);
    let user_info = serde_json::from_str::<UserInfo>(&user_info)
        .context("Failed to parse Facebook user info JSON")?;
    Ok(user_info)
}

#[tracing::instrument(level = "trace", skip(conn))]
pub fn login_db<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    session_user: Option<model::user::User>,
    user_info: UserInfo,
) -> impl std::future::Future<Output = Result<Option<model::user::User>, anyhow::Error>> + Send + 'a
{
    use futures::FutureExt;
    async move {
        let facebook_id = user_info.id.parse::<i64>()
            .context("Failed to parse Facebook ID as i64")?;
        let mut db = conn.acquire().await
            .context("Failed to acquire database connection")?;
        if let Some(user) = session_user {
            log::info!("update account: {:?}", user_info);
            crate::db::user::update_user(
                &mut *db,
                user.user_id,
                Some(crate::db::user::OAuthProvider::Facebook(
                    facebook_id,
                    user_info.name,
                )),
            )
            .await
            .context("Failed to update user with Facebook credentials")?;
            Ok(Some(user))
        } else {
            log::info!("signup: {:?}", user_info);
            let user = crate::db::user::create_user(
                &mut *db,
                crate::db::user::OAuthProvider::Facebook(facebook_id, user_info.name),
            )
            .await
            .context("Failed to create new user with Facebook credentials")?;
            Ok(Some(user))
        }
    }
    .boxed()
}
