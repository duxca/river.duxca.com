pub mod facebook;
pub mod github;

pub(crate) fn oauth_callback_base_url(
    configured_base_url: &str,
    _headers: &axum::http::HeaderMap,
) -> String {
    #[cfg(feature = "local")]
    if let Some(host) = _headers
        .get(axum::http::header::HOST)
        .and_then(|host| host.to_str().ok())
    {
        return format!("http://{host}");
    }

    configured_base_url.to_owned()
}

/// GET /login
/// ひみつのログインページ
#[tracing::instrument(level = "trace", skip(auth_session, st))]
pub async fn login(
    auth_session: axum_login::AuthSession<Backend>,
    axum::extract::State(ref st): axum::extract::State<crate::web::State>,
    req: axum::http::Request<axum::body::Body>,
) -> Result<impl axum::response::IntoResponse, crate::web::Ise> {
    use axum::response::IntoResponse;
    use leptos::prelude::*;

    if auth_session.user.is_some() {
        return Ok(axum::response::Redirect::to("/").into_response());
    }

    let options = st.leptos_options.clone();
    let handler = leptos_axum::render_app_to_stream_with_context(
        || {},
        move || {
            view! {
                <app::LoginPage options=options.clone()/>
            }
        },
    );

    Ok(handler(req).await.into_response())
}

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
    db: sqlx::sqlite::SqlitePool,
    settings: BackendSettings,
}

#[derive(Debug, Clone)]
pub struct BackendSettings {
    pub github_client_id: oauth2::ClientId,
    pub github_client_secret: oauth2::ClientSecret,
    pub github_auth_url: String,
    pub github_token_url: String,
    pub github_user_url: String,
    pub facebook_client_id: oauth2::ClientId,
    pub facebook_client_secret: oauth2::ClientSecret,
    pub facebook_auth_url: String,
    pub facebook_token_url: String,
    pub facebook_user_url: String,
    pub base_url: String,
}

impl Backend {
    #[tracing::instrument(level = "trace", skip(db))]
    pub fn new(db: sqlx::sqlite::SqlitePool, settings: BackendSettings) -> Self {
        Self { db, settings }
    }
}

#[derive(Debug)]
pub enum Credentials {
    Github(github::Credentials),
    Facebook(facebook::Credentials),
}

impl axum_login::AuthnBackend for Backend {
    type User = model::user::User;
    type Credentials = Credentials;
    type Error = BackendError;

    #[tracing::instrument(level = "trace", skip(self))]
    fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> impl std::future::Future<Output = Result<Option<Self::User>, Self::Error>> + Send {
        async move {
            match creds {
                Credentials::Github(creds) => {
                    let access_token = github::get_access_token(
                        self.settings.github_client_id.clone(),
                        self.settings.github_client_secret.clone(),
                        creds.auth_code.clone(),
                        &self.settings.base_url,
                        &self.settings.github_auth_url,
                        &self.settings.github_token_url,
                    )
                    .await?;
                    let user_info =
                        github::get_me(&access_token, &self.settings.github_user_url).await?;
                    let res = github::login_db(&self.db, creds.user, user_info).await?;
                    Ok(res)
                }
                Credentials::Facebook(creds) => {
                    let access_token = facebook::get_access_token(
                        self.settings.facebook_client_id.clone(),
                        self.settings.facebook_client_secret.clone(),
                        creds.auth_code.clone(),
                        &self.settings.base_url,
                        &self.settings.facebook_auth_url,
                        &self.settings.facebook_token_url,
                    )
                    .await?;
                    let user_info =
                        facebook::get_me(&access_token, &self.settings.facebook_user_url).await?;
                    let res = facebook::login_db(&self.db, creds.user, user_info).await?;
                    Ok(res)
                }
            }
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn get_user(
        &self,
        user_id: &axum_login::UserId<Self>,
    ) -> impl std::future::Future<Output = Result<Option<Self::User>, Self::Error>> + Send {
        async move {
            let user = db::user::get_user(&self.db, *user_id).await?;
            Ok(user)
        }
    }
}
