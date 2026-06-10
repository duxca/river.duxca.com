pub mod admin;
pub mod api;
pub mod image;
pub mod login;

#[derive(Clone)]
pub struct State {
    pub db: sqlx::sqlite::SqlitePool,
    pub gcs: google_cloud_storage::client::Storage,
    pub gcs_control: google_cloud_storage::client::StorageControl,
    pub config: crate::Config,
}
impl State {
    pub fn new(
        config: crate::Config,
        db: sqlx::sqlite::SqlitePool,
        gcs: google_cloud_storage::client::Storage,
        gcs_control: google_cloud_storage::client::StorageControl,
    ) -> Result<Self, anyhow::Error> {
        Ok(Self {
            config,
            db,
            gcs,
            gcs_control,
        })
    }
}

pub struct Ise(anyhow::Error);

impl axum::response::IntoResponse for Ise {
    fn into_response(self) -> axum::response::Response {
        log::error!("{:?}", self.0);
        // TODO: 本番環では stack trace を表示しない
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
