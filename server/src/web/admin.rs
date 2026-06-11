use leptos::prelude::{ClassAttribute, ElementChild, RenderHtml};

// GET /admin
#[tracing::instrument(level = "trace", skip(auth_session, st))]
pub async fn admin(
    auth_session: axum_login::AuthSession<crate::web::login::Backend>,
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
    let mut river_waypooints_csv = writer_builder.from_writer(vec![]);
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
            river_waypooints_csv.serialize(model::river::RiverWaypoint::<String>::from(wpt))?;
        }
        river_csv.serialize(model::river::River::<String>::from(river))?;
        river_waypoints.extend(waypoints);
    }
    let body = leptos::prelude::view! {
        <AdminPage data=AdminPageData {
            users,
            user_auths,
            access_logs,
            river_waypoints,
            river_csv_header: "river_id,user_id,river_name,waypoint,description,created_at".to_string(),
            river_csv: String::from_utf8(river_csv.into_inner()?)?,
            river_waypoints_csv_header: "river_waypoint_id,river_id,user_id,waypoint_name,description,waypoint,created_at,updated_at".to_string(),
            river_waypoints_csv: String::from_utf8(river_waypooints_csv.into_inner()?)?,
            river_tracks_csv_header: "river_track_id,river_id,user_id,track_name,description,track,created_at,updated_at".to_string(),
            river_tracks_csv: String::from_utf8(river_tracks_csv.into_inner()?)?,
        }/>
    };
    Ok(axum::response::Html(body.to_html()).into_response())
}

struct AdminPageData {
    users: Vec<model::user::User>,
    user_auths: Vec<model::user::UserAuth>,
    access_logs: Vec<model::user::AccessLog>,
    river_waypoints: Vec<model::river::RiverWaypoint>,
    river_csv_header: String,
    river_csv: String,
    river_waypoints_csv_header: String,
    river_waypoints_csv: String,
    river_tracks_csv_header: String,
    river_tracks_csv: String,
}

const ADMIN_STYLE: &str = r#"
body {
    margin: 0;
    font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    background: #f7f7f5;
    color: #242424;
}
main {
    margin: 0 auto;
    max-width: 1120px;
    padding: 32px 20px 56px;
}
h1 {
    font-size: 28px;
    line-height: 1.2;
    margin: 0 0 24px;
}
h2 {
    font-size: 18px;
    line-height: 1.3;
    margin: 0 0 12px;
}
section {
    border-top: 1px solid #d8d8d2;
    margin-top: 28px;
    padding-top: 24px;
}
.table-wrap {
    overflow-x: auto;
}
table {
    border-collapse: collapse;
    font-size: 14px;
    min-width: 640px;
    width: 100%;
}
th,
td {
    border-bottom: 1px solid #e1e1dc;
    padding: 8px 10px;
    text-align: left;
    vertical-align: top;
}
th {
    background: #ecece8;
    font-weight: 700;
}
textarea {
    box-sizing: border-box;
    font: 14px/1.5 Menlo, Consolas, Monaco, "Lucida Console", monospace;
    min-height: 180px;
    resize: vertical;
    width: 100%;
}
.csv-header {
    color: #5d5d58;
    font-family: Menlo, Consolas, Monaco, "Lucida Console", monospace;
    font-size: 13px;
    margin-bottom: 8px;
    overflow-wrap: anywhere;
}
button {
    appearance: none;
    background: #222;
    border: 1px solid #222;
    border-radius: 6px;
    color: #fff;
    cursor: pointer;
    font: inherit;
    margin-top: 8px;
    padding: 10px 14px;
}
button.secondary {
    background: transparent;
    color: #222;
}
ul {
    list-style: none;
    margin: 0;
    padding: 0;
}
li {
    border-bottom: 1px solid #e1e1dc;
    padding: 8px 0;
}
label {
    display: flex;
    gap: 8px;
}
"#;

#[leptos::prelude::component]
fn AdminPage(data: AdminPageData) -> impl leptos::prelude::IntoView {
    use leptos::prelude::*;

    let AdminPageData {
        users,
        user_auths,
        access_logs,
        river_waypoints,
        river_csv_header,
        river_csv,
        river_waypoints_csv_header,
        river_waypoints_csv,
        river_tracks_csv_header,
        river_tracks_csv,
    } = data;

    view! {
        <!DOCTYPE html>
        <html lang="ja">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <title>"admin"</title>
                <style>{ADMIN_STYLE}</style>
            </head>
            <body>
                <main>
                    <h1>"admin"</h1>
                    <section>
                        <h2>"Users"</h2>
                        <div class="table-wrap">
                            <table>
                                <thead>
                                    <tr>
                                        <th>"user_id"</th>
                                        <th>"nickname"</th>
                                        <th>"role"</th>
                                        <th>"created_at"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {users.into_iter().map(|user| view! {
                                        <tr>
                                            <td>{user.user_id}</td>
                                            <td>{user.nickname}</td>
                                            <td>{user.role}</td>
                                            <td>{user.created_at}</td>
                                        </tr>
                                    }).collect::<Vec<_>>()}
                                </tbody>
                            </table>
                        </div>
                    </section>
                    <section>
                        <h2>"User auths"</h2>
                        <div class="table-wrap">
                            <table>
                                <thead>
                                    <tr>
                                        <th>"user_auth_id"</th>
                                        <th>"user_id"</th>
                                        <th>"identifier"</th>
                                        <th>"identity_type"</th>
                                        <th>"created_at"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {user_auths.into_iter().map(|auth| view! {
                                        <tr>
                                            <td>{auth.user_auth_id}</td>
                                            <td>{auth.user_id}</td>
                                            <td>{auth.identifier}</td>
                                            <td>{auth.identity_type}</td>
                                            <td>{auth.created_at}</td>
                                        </tr>
                                    }).collect::<Vec<_>>()}
                                </tbody>
                            </table>
                        </div>
                    </section>
                    <section>
                        <h2>"Access logs"</h2>
                        <div class="table-wrap">
                            <table>
                                <thead>
                                    <tr>
                                        <th>"access_log_id"</th>
                                        <th>"user_id"</th>
                                        <th>"request"</th>
                                        <th>"created_at"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {access_logs.into_iter().map(|access_log| view! {
                                        <tr>
                                            <td>{access_log.access_log_id}</td>
                                            <td>{access_log.user_id}</td>
                                            <td>{access_log.request}</td>
                                            <td>{access_log.created_at}</td>
                                        </tr>
                                    }).collect::<Vec<_>>()}
                                </tbody>
                            </table>
                        </div>
                    </section>
                    <CsvForm
                        name="river_csv"
                        label="Rivers CSV"
                        header=river_csv_header
                        body=river_csv
                    />
                    <CsvForm
                        name="river_waypoints_csv"
                        label="River waypoints CSV"
                        header=river_waypoints_csv_header
                        body=river_waypoints_csv
                    />
                    <CsvForm
                        name="river_tracks_csv"
                        label="River tracks CSV"
                        header=river_tracks_csv_header
                        body=river_tracks_csv
                    />
                    <section>
                        <h2>"Delete river waypoints"</h2>
                        <form method="post" action="/admin/delete_waypoints">
                            <ul>
                                {river_waypoints.into_iter().map(|wpt| view! {
                                    <li>
                                        <label>
                                            <input
                                                type="checkbox"
                                                name="waypoint_ids"
                                                value=wpt.river_waypoint_id
                                            />
                                            <span>
                                                {wpt.river_waypoint_id}
                                                " / river "
                                                {wpt.river_id}
                                                " / "
                                                {wpt.waypoint_name}
                                            </span>
                                        </label>
                                    </li>
                                }).collect::<Vec<_>>()}
                            </ul>
                            <button type="submit">"Apply"</button>
                        </form>
                    </section>
                    <section>
                        <form method="post" action="/logout">
                            <button class="secondary" type="submit">"Logout"</button>
                        </form>
                    </section>
                </main>
            </body>
        </html>
    }
}

#[leptos::prelude::component]
fn CsvForm(
    name: &'static str,
    label: &'static str,
    header: String,
    body: String,
) -> impl leptos::prelude::IntoView {
    leptos::prelude::view! {
        <section>
            <h2>{label}</h2>
            <form method="post" action="/admin/apply">
                <div class="csv-header">{header}</div>
                <textarea name=name rows="10" cols="50">{body}</textarea>
                <button type="submit">"Apply"</button>
            </form>
        </section>
    };
}

#[derive(Debug, serde::Deserialize)]
pub struct ApplyForm {
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
    }
    Ok(axum::response::Redirect::to("/admin").into_response())
}

#[derive(Debug, serde::Deserialize)]
pub struct ApiForm {
    pub waypoint_ids: Vec<i64>,
}

// POST /admin/delete_waypoints
#[tracing::instrument(level = "trace", skip(auth_session, st))]
pub async fn admin_delete_waypoints(
    auth_session: axum_login::AuthSession<crate::web::login::Backend>,
    axum::extract::State(ref st): axum::extract::State<crate::web::State>,
    axum_extra::extract::Form(ApiForm { waypoint_ids }): axum_extra::extract::Form<ApiForm>,
) -> Result<impl axum::response::IntoResponse, crate::web::Ise> {
    use axum::response::IntoResponse;
    let Some(user) = auth_session.user else {
        return Ok(crate::web::handler_404().await.into_response());
    };
    if user.role != 0 {
        return Ok(crate::web::handler_404().await.into_response());
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
