pub mod facebook;
pub mod github;
pub mod twitter;

/// POST /logout
// #[tracing::instrument(level = "trace")]
pub async fn logout(
    mut auth_session: axum_login::AuthSession<Backend>,
    session: tower_sessions::Session,
    axum::Form(()): axum::Form<()>,
) -> Result<impl axum::response::IntoResponse, crate::web::Ise> {
    auth_session.logout().await?;
    session.flush().await?;
    Ok(axum::response::Redirect::to("/"))
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct BackendError(#[from] pub anyhow::Error);

#[derive(Clone)]
pub struct Backend {
    db: sqlx::SqlitePool,
    settings: BackendSettings,
}

#[derive(Debug, Clone)]
pub struct BackendSettings {
    pub github_client_id: oauth2::ClientId,
    pub github_client_secret: oauth2::ClientSecret,
    pub twitter_client_id: oauth2::ClientId,
    pub twitter_client_secret: oauth2::ClientSecret,
    pub facebook_client_id: oauth2::ClientId,
    pub facebook_client_secret: oauth2::ClientSecret,
    pub base_url: String,
}

impl Backend {
    #[tracing::instrument(level = "trace", skip(db))]
    pub fn new(db: sqlx::SqlitePool, settings: BackendSettings) -> Self {
        Self { db, settings }
    }
}

#[derive(Debug)]
pub enum Credentials {
    Github(github::Credentials),
    Twitter(twitter::Credentials),
    Facebook(facebook::Credentials),
}

#[async_trait::async_trait]
impl axum_login::AuthnBackend for Backend {
    type User = model::user::User;
    type Credentials = Credentials;
    type Error = BackendError;

    // #[tracing::instrument(level = "trace")]
    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        match creds {
            Credentials::Github(creds) => {
                let access_token = github::get_access_token(
                    self.settings.github_client_id.clone(),
                    self.settings.github_client_secret.clone(),
                    creds.auth_code.clone(),
                    &self.settings.base_url,
                )
                .await?;
                let user_info = github::get_me(&access_token).await?;
                let res = github::login_db(&self.db, creds.user, user_info).await?;
                Ok(res)
            }
            Credentials::Twitter(creds) => {
                let access_token = twitter::get_access_token(
                    self.settings.twitter_client_id.clone(),
                    self.settings.twitter_client_secret.clone(),
                    creds.auth_code.clone(),
                    creds.pkce_verifier,
                    &self.settings.base_url,
                )
                .await?;
                let user_info = twitter::get_me(&access_token).await?;
                let res = twitter::login_db(&self.db, creds.user, user_info).await?;
                Ok(res)
            }
            Credentials::Facebook(creds) => {
                let access_token = facebook::get_access_token(
                    self.settings.facebook_client_id.clone(),
                    self.settings.facebook_client_secret.clone(),
                    creds.auth_code.clone(),
                    &self.settings.base_url,
                )
                .await?;
                let user_info = facebook::get_me(&access_token).await?;
                let res = facebook::login_db(&self.db, creds.user, user_info).await?;
                Ok(res)
            }
        }
    }

    // #[tracing::instrument(level = "trace")]
    async fn get_user(
        &self,
        user_id: &axum_login::UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user = crate::db::user::get_user(&self.db, *user_id).await?;
        dbg!(&user);
        Ok(user)
    }
}
