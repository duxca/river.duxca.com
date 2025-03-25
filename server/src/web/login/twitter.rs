// https://docs.x.com/resources/fundamentals/authentication/oauth-2-0/user-access-token
const AUTH_URL: &str = "https://x.com/i/oauth2/authorize";
const TOKEN_URL: &str = "https://api.x.com/2/oauth2/token";
// https://docs.x.com/x-api/users/lookup/quickstart/authenticated-lookup
// https://docs.x.com/x-api/users/user-lookup-me
const USER_URL: &str = "https://api.x.com/2/users/me";
const CSRF_STATE_KEY: &str = "oauth.csrf-state";
// https://developer.x.com/en/portal/projects-and-apps
const REDIRECT_PATH: &str = "/oauth/callback/twitter";

const PKCE_CODE_VERIFIER: &str = "PKCE";

#[derive(Debug, serde::Deserialize)]
pub struct LoginForm {}

/// POST /login/twitter
#[tracing::instrument(level = "trace", skip(auth_session, session))]
pub async fn login(
    auth_session: axum_login::AuthSession<crate::web::login::Backend>,
    session: tower_sessions::Session,
    axum::Form(LoginForm {}): axum::Form<LoginForm>,
) -> Result<impl axum::response::IntoResponse, crate::web::Ise> {
    use anyhow::Context;
    use axum::response::IntoResponse;
    let auth_url = oauth2::AuthUrl::new(AUTH_URL.to_string()).unwrap();
    let token_url = oauth2::TokenUrl::new(TOKEN_URL.to_string()).unwrap();
    let client_id = auth_session.backend.settings.twitter_client_id.clone();
    let client_secret = auth_session.backend.settings.twitter_client_secret.clone();
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
    let (pkce_code, pkce_verifier) = oauth2::PkceCodeChallenge::new_random_plain();
    // https://docs.x.com/resources/fundamentals/authentication/guides/v2-authentication-mapping
    let (auth_url, csrf_state) = client
        .authorize_url(oauth2::CsrfToken::new_random)
        .add_scope(oauth2::Scope::new("users.read".to_string()))
        .add_scope(oauth2::Scope::new("tweet.read".to_string()))
        .add_scope(oauth2::Scope::new("follows.write".to_string()))
        .set_pkce_challenge(pkce_code.clone())
        .url();
    session
        .insert(CSRF_STATE_KEY, csrf_state.secret())
        .await
        .context("CSRFトークンの保存に失敗")?;
    session
        .insert(PKCE_CODE_VERIFIER, pkce_verifier.secret())
        .await
        .context("PKCEベリファイアの保存に失敗")?;
    session.save().await.context("セッションの保存に失敗")?;

    //https://x.com/i/oauth2/authorize?
    //response_type=code&
    //client_id=M1M5R3BMVy13QmpScXkzTUt5OE46MTpjaQ&
    //redirect_uri=https://www.example.com&
    //scope=tweet.read%20users.read%20follows.read%20follows.write
    //&state=state&
    //code_challenge=challenge&
    //code_challenge_method=plain

    //https://x.com/i/oauth2/authorize?
    //response_type=code&
    //client_id=clQ4WkVSVTZnOVJRRF96alpOb286MTpjaQ&
    //state=yCY5VEuOD1iNrNDcyJW-DA&
    //code_challenge=AB2hd9LInzB6Y7qZRNl8pFeqS_FIUtepHT-6oAgkKlU&
    //code_challenge_method=plain&
    //redirect_uri=https%3A%2F%2Friver.duxca.com%2Foauth%2Fcallback%2Ftwitter&scope=users.read
    Ok(axum::response::Redirect::to(auth_url.as_str()).into_response())
}

// OAuth2 の認可コードを受け取るためのクエリパラメータ
#[derive(Debug, Clone, serde::Deserialize)]
pub struct AuthzRequestQuery {
    pub code: oauth2::AuthorizationCode,
    pub state: oauth2::CsrfToken,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Credentials {
    pub auth_code: oauth2::AuthorizationCode,
    pub pkce_verifier: oauth2::PkceCodeVerifier,
    pub user: Option<model::user::User>,
}

/// GET /oauth/callback/twitter
#[tracing::instrument(level = "trace", skip(auth_session, session))]
pub async fn callback(
    mut auth_session: axum_login::AuthSession<crate::web::login::Backend>,
    session: tower_sessions::Session,
    axum::extract::Query(AuthzRequestQuery {
        code: auth_code,
        state: incomming_state,
    }): axum::extract::Query<AuthzRequestQuery>,
) -> Result<impl axum::response::IntoResponse, crate::web::Ise> {
    use anyhow::Context;
    use axum::response::IntoResponse;
    // セッションがない場合はエラー
    let Some(saved_state) = session.get::<oauth2::CsrfToken>(CSRF_STATE_KEY).await? else {
        log::error!("cannot find csrf state");
        return Ok((axum::http::StatusCode::BAD_REQUEST, "session expired").into_response());
    };
    let Some(pkce_verifier) = session
        .get::<oauth2::PkceCodeVerifier>(PKCE_CODE_VERIFIER)
        .await
        .context("PKCEベリファイアの取得に失敗")?
    else {
        log::error!("cannot find pkce verifier");
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
    let creds = super::Credentials::Twitter(Credentials {
        auth_code,
        pkce_verifier,
        // ログイン済みかどうか
        user: auth_session.user.clone(),
    });
    let Some(user) = auth_session
        .authenticate(creds)
        .await
        .context("認証処理に失敗")?
    else {
        return Ok((
            axum::http::StatusCode::UNAUTHORIZED,
            "authentication failed",
        )
            .into_response());
    };
    auth_session
        .login(&user)
        .await
        .context("セッションへのユーザーログインに失敗")?;
    Ok(axum::response::Redirect::to("/").into_response())
}

#[tracing::instrument(level = "trace")]
pub async fn get_access_token(
    client_id: oauth2::ClientId,
    client_secret: oauth2::ClientSecret,
    auth_code: oauth2::AuthorizationCode,
    pkce_verifier: oauth2::PkceCodeVerifier,
    base_url: &str,
) -> Result<oauth2::AccessToken, anyhow::Error> {
    use anyhow::Context;
    let auth_url = oauth2::AuthUrl::new(AUTH_URL.to_string()).unwrap();
    let token_url = oauth2::TokenUrl::new(TOKEN_URL.to_string()).unwrap();
    let redirect_url = oauth2::RedirectUrl::new(format!("{}{}", base_url, REDIRECT_PATH)).unwrap();
    let client = oauth2::basic::BasicClient::new(client_id)
        .set_client_secret(client_secret)
        .set_auth_uri(auth_url)
        .set_token_uri(token_url)
        .set_redirect_uri(redirect_url);
    let token_res = client
        .exchange_code(auth_code)
        .set_pkce_verifier(pkce_verifier)
        .request_async(&reqwest::Client::new())
        .await
        .context("Twitterアクセストークンの取得に失敗")?;
    use oauth2::TokenResponse;
    let access_token = token_res.access_token().clone();
    Ok(access_token)
}
#[derive(Debug, serde::Deserialize)]
pub struct UserInfoWrapper {
    pub data: UserInfo,
}
#[derive(Debug, serde::Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub name: String,
    pub username: String,
}

#[tracing::instrument(level = "trace")]
pub async fn get_me(access_token: &oauth2::AccessToken) -> Result<UserInfo, anyhow::Error> {
    use anyhow::Context;
    let res = reqwest::Client::new()
        .get(USER_URL)
        .header(
            axum::http::header::AUTHORIZATION.as_str(),
            format!("Bearer {}", access_token.secret().as_str()),
        )
        .header(axum::http::header::USER_AGENT.as_str(), "axum-login")
        .send()
        .await
        .context("Twitterユーザー情報の取得リクエストに失敗")?;
    let user_info = res
        .text()
        .await
        .context("Twitterユーザー情報のレスポンス取得に失敗")?;
    log::debug!("{}", user_info);
    let user_info = serde_json::from_str::<UserInfoWrapper>(&user_info)
        .context("Twitterユーザー情報のJSONパースに失敗")?;
    Ok(user_info.data)
}

#[tracing::instrument(level = "trace", skip(conn))]
pub fn login_db<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    session_user: Option<model::user::User>,
    user_info: UserInfo,
) -> impl std::future::Future<Output = Result<Option<model::user::User>, anyhow::Error>> + Send + 'a
{
    use anyhow::Context;
    use futures::FutureExt;
    use std::str::FromStr;
    async move {
        let mut db = conn
            .acquire()
            .await
            .context("データベース接続の取得に失敗")?;
        if let Some(user) = session_user {
            crate::db::user::update_user(
                &mut *db,
                user.user_id,
                Some(crate::db::user::OAuthProvider::Twitter(
                    user_info.id.parse().unwrap(),
                    user_info.name,
                )),
            )
            .await
            .map_err(|o| dbg!(o))?;
            Ok(Some(user))
        } else {
            log::info!("signup: {:?}", user_info.username);
            let user = crate::db::user::create_user(
                &mut *db,
                crate::db::user::OAuthProvider::Twitter(
                    user_info.id.parse().unwrap(),
                    user_info.name,
                ),
            )
            .await
            .map_err(|o| dbg!(o))?;
            dbg!(&user);
            Ok(Some(user))
        }
    }
    .boxed()
}
