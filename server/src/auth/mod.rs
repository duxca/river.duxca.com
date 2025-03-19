pub mod facebook;
pub mod github;
pub mod twitter;

#[derive(Debug, Clone)]
pub struct ClientToken {
    pub client_id: oauth2::ClientId,
    pub client_secret: oauth2::ClientSecret,
}

#[derive(Debug, Clone)]
pub struct Credentials {
    pub code: String,
    pub old_state: oauth2::CsrfToken,
    pub new_state: oauth2::CsrfToken,
    pub provider: OAuthProvider,
    pub user: Option<model::user::User>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OAuthProvider {
    Github,
    Twitter,
    Facebook,
}
impl std::str::FromStr for OAuthProvider {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "github" => Ok(Self::Github),
            "twitter" => Ok(Self::Twitter),
            "facebook" => Ok(Self::Facebook),
            _ => Err(anyhow::anyhow!("invalid OAuth provider: {}", s)),
        }
    }
}
impl std::fmt::Display for OAuthProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Github => write!(f, "github"),
            Self::Twitter => write!(f, "twitter"),
            Self::Facebook => write!(f, "facebook"),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct BackendError(#[from] pub anyhow::Error);

#[derive(Clone)]
pub struct Backend {
    db: sqlx::SqlitePool,
    github: github::Backend,
    twitter: twitter::Backend,
    facebook: facebook::Backend,
}

#[derive(Debug, Clone)]
pub struct BackendSettings {
    pub github: ClientToken,
    pub twitter: ClientToken,
    pub facebook: ClientToken,
    pub redirect_url: oauth2::RedirectUrl,
}

impl Backend {
    // #[tracing::instrument(level = "trace", skip(db))]
    pub fn new(db: sqlx::SqlitePool, settings: BackendSettings) -> Self {
        Self {
            db: db.clone(),
            github: github::Backend::new(
                db.clone(),
                settings.github,
                settings.redirect_url.clone(),
            ),
            twitter: twitter::Backend::new(
                db.clone(),
                settings.twitter,
                settings.redirect_url.clone(),
            ),
            facebook: facebook::Backend::new(
                db.clone(),
                settings.facebook,
                settings.redirect_url.clone(),
            ),
        }
    }

    // #[tracing::instrument(level = "trace")]
    pub fn authorize_url(&self, provider: OAuthProvider) -> (oauth2::url::Url, oauth2::CsrfToken) {
        match provider {
            OAuthProvider::Github => self.github.authorize_url(),
            OAuthProvider::Twitter => self.twitter.authorize_url(),
            OAuthProvider::Facebook => self.facebook.authorize_url(),
        }
    }
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
        match creds.provider {
            OAuthProvider::Github => self.github.authenticate(creds).await,
            OAuthProvider::Twitter => self.twitter.authenticate(creds).await,
            OAuthProvider::Facebook => self.facebook.authenticate(creds).await,
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
