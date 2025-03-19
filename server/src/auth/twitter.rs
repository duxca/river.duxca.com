// https://developer.x.com/ja/docs/authentication/api-reference/authorize
const AUTH_URL: &str = "https://api.x.com/oauth/authorize";
const TOKEN_URL: &str = "https://api.x.com/oauth/access_token";
// https://docs.x.com/x-api/users/lookup/quickstart/authenticated-lookup
const USER_URL: &str = "https://api.x.com/2/users/me";


#[derive(Debug, serde::Deserialize)]
pub struct UserInfo {
    id: i64,
    name: String,
    username: String,
}

async fn get_me(access_token: &str) -> Result<UserInfo, anyhow::Error> {
    let res = reqwest::Client::new()
        .get(USER_URL)
        .header(
            axum::http::header::AUTHORIZATION.as_str(),
            format!("Bearer {access_token}"),
        )
        .header(axum::http::header::USER_AGENT.as_str(), "axum-login")
        .send()
        .await;
    let user_info = res
        .map_err(anyhow::Error::from)?
        .text()
        .await
        .map_err(anyhow::Error::from)?;
    log::debug!("{}", user_info);
    let user_info = serde_json::from_str::<UserInfo>(&user_info).map_err(anyhow::Error::from)?;
    Ok(user_info)
}

fn login<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    session_user: Option<model::user::User>,
    user_info: UserInfo,
) -> impl std::future::Future<Output = Result<Option<model::user::User>, anyhow::Error>> + Send + 'a
{
    use futures::FutureExt;
    async move {
        let mut db = conn.acquire().await.map_err(anyhow::Error::from)?;
        if let Some(user) = session_user {
            crate::db::user::update_user(
                &mut *db,
                user.user_id,
                Some(crate::db::user::OAuthProvider::Twitter(
                    user_info.id,
                    user_info.name,
                )),
            )
            .await
            .map_err(|o| dbg!(o))?;
            Ok(Some(user))
        } else {
            log::info!("signup: {:?}", user_info);
            let user = crate::db::user::create_user(
                &mut *db,
                crate::db::user::OAuthProvider::Twitter(user_info.id, user_info.name),
            )
            .await
            .map_err(|o| dbg!(o))?;
            dbg!(&user);
            Ok(Some(user))
        }
    }.boxed()
}

#[derive(Clone)]
pub struct Backend {
    db: sqlx::SqlitePool,
    client_token: super::ClientToken,
    redirect_url: oauth2::RedirectUrl,
}

impl Backend {
    pub fn new(
        db: sqlx::SqlitePool,
        client_token: super::ClientToken,
        redirect_url: oauth2::RedirectUrl,
    ) -> Self {
        Self {
            db,
            client_token,
            redirect_url,
        }
    }

    pub fn authorize_url(&self) -> (oauth2::url::Url, oauth2::CsrfToken) {
        let auth_url = oauth2::AuthUrl::new(AUTH_URL.to_string()).unwrap();
        let token_url = oauth2::TokenUrl::new(TOKEN_URL.to_string()).unwrap();
        let client = oauth2::basic::BasicClient::new(self.client_token.client_id.clone())
            .set_client_secret(self.client_token.client_secret.clone())
            .set_auth_uri(auth_url)
            .set_token_uri(token_url)
            .set_redirect_uri(self.redirect_url.clone());
        client.authorize_url(oauth2::CsrfToken::new_random).url()
    }
}

#[async_trait::async_trait]
impl axum_login::AuthnBackend for Backend {
    type User = model::user::User;
    type Credentials = super::Credentials;
    type Error = super::BackendError;

    // #[tracing::instrument]
    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        use oauth2::TokenResponse;
        // Ensure the CSRF state has not been tampered with.
        if creds.old_state.secret() != creds.new_state.secret() {
            return Ok(None);
        };
        let auth_url = oauth2::AuthUrl::new(AUTH_URL.to_string()).unwrap();
        let token_url = oauth2::TokenUrl::new(TOKEN_URL.to_string()).unwrap();
        let client = oauth2::basic::BasicClient::new(self.client_token.client_id.clone())
            .set_client_secret(self.client_token.client_secret.clone())
            .set_auth_uri(auth_url)
            .set_token_uri(token_url)
            .set_redirect_uri(self.redirect_url.clone());
        // Process authorization code, expecting a token response back.
        let token_res = client
            .exchange_code(oauth2::AuthorizationCode::new(creds.code))
            .request_async(&reqwest::Client::new())
            .await
            .map_err(anyhow::Error::from)?;
        let user_info = get_me(token_res.access_token().secret()).await?;
        let res = login(&self.db, creds.user, user_info).await?;
        Ok(res)
    }

    // #[tracing::instrument]
    async fn get_user(
        &self,
        user_id: &axum_login::UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user = crate::db::user::get_user(&self.db, *user_id).await?;
        dbg!(&user);
        Ok(user)
    }
}
