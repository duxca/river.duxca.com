pub mod facebook;
pub mod github;

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

    let mut conn = st.db.acquire().await?;
    let user = auth_session.user;
    let auths = if let Some(user) = user.as_ref() {
        db::user::get_user_auths(&mut conn, user.user_id).await?
    } else {
        vec![]
    };
    let providers = crate::web::ui::AuthProviders::from_auths(&auths);
    let handler = leptos_axum::render_app_to_stream_with_context(
        || {},
        move || {
            view! {
                <crate::web::ui::LoginPage user=user.clone() providers=providers.clone()/>
            }
        },
    );

    Ok(handler(req).await.into_response())
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
                    )
                    .await?;
                    let user_info = github::get_me(&access_token).await?;
                    let res = github::login_db(&self.db, creds.user, user_info).await?;
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
