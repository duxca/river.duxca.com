// GET /admin
#[tracing::instrument(level = "trace", skip(auth_session, st))]
pub async fn admin(
    auth_session: axum_login::AuthSession<crate::web::login::Backend>,
    axum::extract::State(ref st): axum::extract::State<crate::web::State>,
) -> Result<impl axum::response::IntoResponse, crate::web::Ise> {
    use askama::Template;
    use axum::response::IntoResponse;
    #[derive(Debug, askama::Template)]
    #[template(path = "admin.html")]
    struct Tmpl {
        users: Vec<model::user::User>,
        user_auths: Vec<model::user::UserAuth>,
        access_logs: Vec<model::user::AccessLog>,
        river_csv: String,
        river_waypoints_csv: String,
        river_tracks_csv: String,
    }
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
    let mut river_csv = csv::Writer::from_writer(vec![]);
    let mut river_tracks_csv = csv::Writer::from_writer(vec![]);
    let mut river_waypooints_csv = csv::Writer::from_writer(vec![]);
    let rivers = db::rivers::list_rivers_all(&mut conn).await?;
    for river in &rivers {
        river_csv.serialize(river)?;
        let tracks = db::river_tracks::list_river_tracks_all(&mut conn, river.river_id).await?;
        for track in tracks {
            river_tracks_csv.serialize(track)?;
        }
        let waypoints =
            db::river_waypoints::list_river_waypoints_all(&mut conn, river.river_id).await?;
        for waypoint in waypoints {
            let wpt: Vec<f64> = serde_json::from_value(waypoint.waypoint)?;
            river_waypooints_csv.serialize(wpt)?;
        }
    }
    let template = Tmpl {
        users,
        user_auths,
        access_logs,
        river_csv: String::from_utf8(river_csv.into_inner()?)?,
        river_waypoints_csv: String::from_utf8(river_waypooints_csv.into_inner()?)?,
        river_tracks_csv: String::from_utf8(river_tracks_csv.into_inner()?)?,
    };
    let body = axum::response::Html(template.render()?);
    Ok(body.into_response())
}

#[derive(Debug, serde::Deserialize)]
struct ApplyForm {
    river_csv: Option<String>,
    river_waypoints_csv: Option<String>,
    river_tracks_csv: Option<String>,
}

// POST /admin/apply
#[tracing::instrument(level = "trace", skip(auth_session, st))]
pub async fn admin_apply(
    auth_session: axum_login::AuthSession<crate::web::login::Backend>,
    axum::extract::State(ref st): axum::extract::State<crate::web::State>,
    axum::extract::Form(ApplyForm {
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
    let mut conn = st.db.acquire().await?;
    if let Some(river_csv) = river_csv {
        let mut rdr = csv::Reader::from_reader(river_csv.as_bytes());
        for result in rdr.deserialize::<model::river::River>() {
            let model::river::River {
                river_id,
                user_id,
                river_name,
                waypoint,
                description,
                created_at,
            } = result?;
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
        let mut rdr = csv::Reader::from_reader(river_waypoints_csv.as_bytes());
        for result in rdr.deserialize::<model::river::RiverWaypoint>() {
            let model::river::RiverWaypoint {
                river_waypoint_id,
                river_id,
                user_id,
                waypoint_name,
                description,
                waypoint,
                created_at,
                updated_at,
            } = result?;
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
        if let Some(river_tracks_csv) = river_tracks_csv {
            let mut rdr = csv::Reader::from_reader(river_tracks_csv.as_bytes());
            for result in rdr.deserialize::<model::river::RiverTrack>() {
                let model::river::RiverTrack {
                    river_track_id,
                    river_id,
                    user_id,
                    track_name,
                    description,
                    track,
                    created_at,
                    updated_at,
                } = result?;
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
    }
    Ok(axum::response::Redirect::to("/admin").into_response())
}
