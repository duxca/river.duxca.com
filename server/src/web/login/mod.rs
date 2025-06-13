pub mod facebook;
pub mod github;
pub mod twitter;

/// GET /login
/// ひみつのログインページ
#[tracing::instrument(level = "trace", skip(auth_session, st))]
pub async fn login(
    auth_session: axum_login::AuthSession<Backend>,
    axum::extract::State(ref st): axum::extract::State<crate::web::State>,
) -> Result<impl axum::response::IntoResponse, crate::web::Ise> {
    use askama::Template;
    use axum::response::IntoResponse;
    let mut conn = st.db.acquire().await?;
    let auths = if let Some(user) = auth_session.user {
        db::user::get_user_auths(&mut conn, user.user_id).await?
    } else {
        vec![]
    };
    #[derive(Debug, askama::Template)]
    #[template(path = "login.html")]
    struct Tmpl {
        github: Option<model::user::UserAuth>,
        twitter: Option<model::user::UserAuth>,
        facebook: Option<model::user::UserAuth>,
    }
    let template = Tmpl {
        github: auths
            .iter()
            .find(|a| a.identity_type == 0)
            .map(ToOwned::to_owned),
        facebook: auths
            .iter()
            .find(|a| a.identity_type == 1)
            .map(ToOwned::to_owned),
        twitter: auths
            .iter()
            .find(|a| a.identity_type == 2)
            .map(ToOwned::to_owned),
    };
    let body = axum::response::Html(template.render()?);
    Ok(body.into_response())
}

/// GET /logout
/// POST /logout
#[tracing::instrument(level = "trace", skip(auth_session, session))]
pub async fn logout(
    mut auth_session: axum_login::AuthSession<Backend>,
    session: tower_sessions::Session,
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

    #[tracing::instrument(level = "trace", skip(self))]
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

    #[tracing::instrument(level = "trace", skip(self))]
    async fn get_user(
        &self,
        user_id: &axum_login::UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user = db::user::get_user(&self.db, *user_id).await?;
        dbg!(&user);
        Ok(user)
    }
}
