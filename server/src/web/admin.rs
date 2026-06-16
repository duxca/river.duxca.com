use leptos::prelude::RenderHtml;

const ADMIN_CSRF_TOKEN_KEY: &str = "admin.csrf-token";

async fn admin_csrf_token(session: &tower_sessions::Session) -> Result<String, crate::web::Ise> {
    use anyhow::Context;

    if let Some(token) = session
        .get::<String>(ADMIN_CSRF_TOKEN_KEY)
        .await
        .context("Failed to get admin CSRF token from session")?
    {
        return Ok(token);
    }

    let token = oauth2::CsrfToken::new_random().secret().to_string();
    session
        .insert(ADMIN_CSRF_TOKEN_KEY, &token)
        .await
        .context("Failed to insert admin CSRF token into session")?;
    session
        .save()
        .await
        .context("Failed to save session after admin CSRF token insertion")?;
    Ok(token)
}

async fn validate_admin_csrf_token(
    session: &tower_sessions::Session,
    csrf_token: &str,
) -> Result<bool, crate::web::Ise> {
    use anyhow::Context;

    let saved_token = session
        .get::<String>(ADMIN_CSRF_TOKEN_KEY)
        .await
        .context("Failed to get admin CSRF token from session")?;
    Ok(saved_token.as_deref() == Some(csrf_token))
}

// GET /admin
#[tracing::instrument(level = "trace", skip(auth_session, session, st))]
pub async fn admin(
    auth_session: axum_login::AuthSession<crate::web::login::Backend>,
    session: tower_sessions::Session,
    axum::extract::State(ref st): axum::extract::State<crate::web::State>,
) -> Result<impl axum::response::IntoResponse, crate::web::Ise> {
    use axum::response::IntoResponse;
    let Some(user) = auth_session.user else {
        return Ok(crate::web::handler_404().await.into_response());
    };
    if user.role != 0 {
        return Ok(crate::web::handler_404().await.into_response());
    }
    let mut conn = st.db.acquire().await?;
    let (access_logs, _, _) = db::user::list_access_logs(&mut conn, None, 0, 100).await?;
    let (users, _, _) = db::user::list_users(&mut conn, 0, 100).await?;
    let (user_auths, _) = db::user::list_user_auths(&mut conn, 0, 100).await?;
    let mut writer_builder = csv::WriterBuilder::new();
    writer_builder
        .delimiter(b',')
        .has_headers(false)
        .quote(b'\\');
    let mut river_csv = writer_builder.from_writer(vec![]);
    let mut river_tracks_csv = writer_builder.from_writer(vec![]);
    let mut river_waypoints_csv = writer_builder.from_writer(vec![]);
    let rivers = db::rivers::list_rivers_all(&mut conn).await?;
    let mut river_waypoints = vec![];
    for river in rivers {
        let tracks = db::river_tracks::list_river_tracks_all(&mut conn, river.river_id).await?;
        for track in tracks {
            river_tracks_csv.serialize(model::river::RiverTrack::<String>::from(track))?;
        }
        let waypoints =
            db::river_waypoints::list_river_waypoints_all(&mut conn, river.river_id).await?;
        for wpt in waypoints.clone() {
            river_waypoints_csv.serialize(model::river::RiverWaypoint::<String>::from(wpt))?;
        }
        river_csv.serialize(model::river::River::<String>::from(river))?;
        river_waypoints.extend(waypoints);
    }
    let csrf_token = admin_csrf_token(&session).await?;
    let body = leptos::prelude::view! {
        <app::AdminPage data=app::AdminPageData {
            users,
            user_auths,
            access_logs,
            river_waypoints,
            csrf_token,
            river_csv_header: "river_id,user_id,river_name,waypoint,description,created_at".to_string(),
            river_csv: String::from_utf8(river_csv.into_inner()?)?,
            river_waypoints_csv_header: "river_waypoint_id,river_id,user_id,waypoint_name,description,waypoint,created_at,updated_at".to_string(),
            river_waypoints_csv: String::from_utf8(river_waypoints_csv.into_inner()?)?,
            river_tracks_csv_header: "river_track_id,river_id,user_id,track_name,description,track,created_at,updated_at".to_string(),
            river_tracks_csv: String::from_utf8(river_tracks_csv.into_inner()?)?,
        }/>
    };
    Ok(axum::response::Html(body.to_html()).into_response())
}

#[derive(Debug, serde::Deserialize)]
pub struct ApplyForm {
    csrf_token: String,
    river_csv: Option<String>,
    river_waypoints_csv: Option<String>,
    river_tracks_csv: Option<String>,
}

// POST /admin/apply
#[tracing::instrument(level = "trace", skip(auth_session, session, st))]
pub async fn admin_apply(
    auth_session: axum_login::AuthSession<crate::web::login::Backend>,
    session: tower_sessions::Session,
    axum::extract::State(ref st): axum::extract::State<crate::web::State>,
    axum::extract::Form(ApplyForm {
        csrf_token,
        river_csv,
        river_waypoints_csv,
        river_tracks_csv,
    }): axum::extract::Form<ApplyForm>,
) -> Result<impl axum::response::IntoResponse, crate::web::Ise> {
    use axum::response::IntoResponse;
    let Some(user) = auth_session.user else {
        return Ok(crate::web::handler_404().await.into_response());
    };
    if user.role != 0 {
        return Ok(crate::web::handler_404().await.into_response());
    }
    if !validate_admin_csrf_token(&session, &csrf_token).await? {
        return Ok((axum::http::StatusCode::BAD_REQUEST, "invalid csrf token").into_response());
    }
    let mut reader_builder = csv::ReaderBuilder::new();
    reader_builder
        .delimiter(b',')
        .quote(b'\\')
        .has_headers(false);
    let mut conn = st.db.acquire().await?;
    if let Some(river_csv) = river_csv {
        let mut rdr = reader_builder.from_reader(river_csv.as_bytes());
        for result in rdr.deserialize::<model::river::River<String>>() {
            let model::river::River {
                river_id,
                user_id,
                river_name,
                waypoint,
                description,
                created_at,
            } = model::river::River::<serde_json::Value>::try_from(result?)?;
            sqlx::query!(
                r#"
            INSERT INTO rivers (river_id, user_id, river_name, waypoint, description, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            ON CONFLICT (river_id)
            DO UPDATE SET
                user_id = ?2,
                river_name = ?3,
                waypoint = ?4,
                description = ?5,
                created_at = ?6
            RETURNING river_id;
            "#,
                river_id,
                user_id,
                river_name,
                waypoint,
                description,
                created_at,
            )
            .fetch_one(&mut *conn)
            .await?;
        }
    }
    if let Some(river_waypoints_csv) = river_waypoints_csv {
        let mut rdr = reader_builder.from_reader(river_waypoints_csv.as_bytes());
        for result in rdr.deserialize::<model::river::RiverWaypoint<String>>() {
            let model::river::RiverWaypoint {
                river_waypoint_id,
                river_id,
                user_id,
                waypoint_name,
                description,
                waypoint,
                created_at,
                updated_at,
            } = model::river::RiverWaypoint::<serde_json::Value>::try_from(result?)?;
            sqlx::query!(
                r#"
            INSERT INTO river_waypoints (river_waypoint_id, river_id, user_id, waypoint_name, description, waypoint, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            ON CONFLICT (river_waypoint_id)
            DO UPDATE SET
                river_id = ?2,
                user_id = ?3,
                waypoint_name = ?4,
                description = ?5,
                waypoint = ?6,
                created_at = ?7,
                updated_at = ?8
            RETURNING river_waypoint_id;
            "#,
                river_waypoint_id,
                river_id,
                user_id,
                waypoint_name,
                description,
                waypoint,
                created_at,
                updated_at,
            )
            .fetch_one(&mut *conn)
            .await?;
        }
    }
    if let Some(river_tracks_csv) = river_tracks_csv {
        let mut rdr = reader_builder.from_reader(river_tracks_csv.as_bytes());
        for result in rdr.deserialize::<model::river::RiverTrack<String>>() {
            let model::river::RiverTrack {
                river_track_id,
                river_id,
                user_id,
                track_name,
                description,
                track,
                created_at,
                updated_at,
            } = model::river::RiverTrack::<serde_json::Value>::try_from(result?)?;
            sqlx::query!(
                r#"
            INSERT INTO river_tracks (river_track_id, river_id, user_id, track_name, description, track, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            ON CONFLICT (river_track_id)
            DO UPDATE SET
                river_id = ?2,
                user_id = ?3,
                track_name = ?4,
                description = ?5,
                track = ?6,
                created_at = ?7,
                updated_at = ?8
            RETURNING river_track_id;
            "#,
                river_track_id,
                river_id,
                user_id,
                track_name,
                description,
                track,
                created_at,
                updated_at
            )
            .fetch_one(&mut *conn)
            .await?;
        }
    }
    Ok(axum::response::Redirect::to("/admin").into_response())
}

#[derive(Debug, serde::Deserialize)]
pub struct ApiForm {
    pub csrf_token: String,
    pub waypoint_ids: Vec<i64>,
}

// POST /admin/delete_waypoints
#[tracing::instrument(level = "trace", skip(auth_session, session, st))]
pub async fn admin_delete_waypoints(
    auth_session: axum_login::AuthSession<crate::web::login::Backend>,
    session: tower_sessions::Session,
    axum::extract::State(ref st): axum::extract::State<crate::web::State>,
    axum_extra::extract::Form(ApiForm {
        csrf_token,
        waypoint_ids,
    }): axum_extra::extract::Form<ApiForm>,
) -> Result<impl axum::response::IntoResponse, crate::web::Ise> {
    use axum::response::IntoResponse;
    let Some(user) = auth_session.user else {
        return Ok(crate::web::handler_404().await.into_response());
    };
    if user.role != 0 {
        return Ok(crate::web::handler_404().await.into_response());
    }
    if !validate_admin_csrf_token(&session, &csrf_token).await? {
        return Ok((axum::http::StatusCode::BAD_REQUEST, "invalid csrf token").into_response());
    }
    for wpt in waypoint_ids {
        service::handler(
            &st.db,
            &user,
            model::api::delete_river_waypoint::Request {
                river_waypoint_id: wpt,
            }
            .into(),
        )
        .await?;
    }
    Ok(axum::response::Redirect::to("/admin").into_response())
}
