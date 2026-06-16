pub mod admin;
pub mod app;
#[cfg(feature = "local")]
pub mod fake_facebook;
#[cfg(feature = "local")]
pub mod fake_github;
pub mod home;
pub mod login;
pub mod server_fn;

#[derive(Clone)]
pub struct State {
    pub db: sqlx::sqlite::SqlitePool,
    #[allow(dead_code)]
    pub config: crate::Config,
    pub leptos_options: leptos::config::LeptosOptions,
}
impl State {
    pub fn new(
        config: crate::Config,
        db: sqlx::sqlite::SqlitePool,
        leptos_options: leptos::config::LeptosOptions,
    ) -> Result<Self, anyhow::Error> {
        Ok(Self {
            config,
            db,
            leptos_options,
        })
    }
}

pub struct Ise(anyhow::Error);

impl axum::response::IntoResponse for Ise {
    fn into_response(self) -> axum::response::Response {
        log::error!("{:?}", self.0);
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {:?}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for Ise
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

pub async fn handler_404() -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, "404 not found")
}
