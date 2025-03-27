pub mod api;
pub mod login;

#[derive(Clone)]
pub struct State {
    pub db: sqlx::sqlite::SqlitePool,
}
impl State {
    pub fn from_pool(pool: sqlx::sqlite::SqlitePool) -> Result<Self, anyhow::Error> {
        Ok(Self { db: pool })
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

#[derive(Debug, serde::Deserialize)]
pub struct LoginForm {}

// GET /admin
#[tracing::instrument(level = "trace", skip(auth_session, st))]
pub async fn admin(
    auth_session: axum_login::AuthSession<crate::web::login::Backend>,
    axum::extract::State(ref st): axum::extract::State<crate::web::State>,
) -> Result<impl axum::response::IntoResponse, Ise> {
    use askama::Template;
    use axum::response::IntoResponse;
    if let Some(user) = auth_session.user {
        let mut conn = st.db.acquire().await?;

        let (access_logs, _, _) =
            db::user::list_access_logs(&mut conn, Some(user.user_id), 0, 100).await?;
        #[derive(Debug, askama::Template)]
        #[template(path = "admin.html")]
        struct Tmpl {
            users: Vec<model::user::User>,
            access_logs: Vec<model::user::AccessLog>,
        }
        let users = vec![user];
        let template = Tmpl { users, access_logs };
        let body = axum::response::Html(template.render()?);
        return Ok(body.into_response());
    } else {
        #[derive(Debug, askama::Template)]
        #[template(path = "login.html")]
        struct Tmpl {
            redirect: String,
        }
        let template = Tmpl {
            redirect: "/admin".to_string(),
        };
        let body = axum::response::Html(template.render()?);
        Ok(body.into_response())
    }
}
