use leptos::prelude::*;

pub struct AdminPageData {
    pub users: Vec<model::user::User>,
    pub user_auths: Vec<model::user::UserAuth>,
    pub access_logs: Vec<model::user::AccessLog>,
    pub river_waypoints: Vec<model::river::RiverWaypoint>,
    pub csrf_token: String,
    pub river_csv_header: String,
    pub river_csv: String,
    pub river_waypoints_csv_header: String,
    pub river_waypoints_csv: String,
    pub river_tracks_csv_header: String,
    pub river_tracks_csv: String,
}

const ADMIN_CSRF_FORM_FIELD: &str = "csrf_token";

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

#[component]
pub fn AdminPage(data: AdminPageData) -> impl IntoView {
    let AdminPageData {
        users,
        user_auths,
        access_logs,
        river_waypoints,
        csrf_token,
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
                        csrf_token=csrf_token.clone()
                        header=river_csv_header
                        body=river_csv
                    />
                    <CsvForm
                        name="river_waypoints_csv"
                        label="River waypoints CSV"
                        csrf_token=csrf_token.clone()
                        header=river_waypoints_csv_header
                        body=river_waypoints_csv
                    />
                    <CsvForm
                        name="river_tracks_csv"
                        label="River tracks CSV"
                        csrf_token=csrf_token.clone()
                        header=river_tracks_csv_header
                        body=river_tracks_csv
                    />
                    <section>
                        <h2>"Delete river waypoints"</h2>
                        <form method="post" action="/admin/delete_waypoints">
                            <input
                                type="hidden"
                                name=ADMIN_CSRF_FORM_FIELD
                                value=csrf_token
                            />
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

#[component]
fn CsvForm(
    name: &'static str,
    label: &'static str,
    csrf_token: String,
    header: String,
    body: String,
) -> impl IntoView {
    view! {
        <section>
            <h2>{label}</h2>
            <form method="post" action="/admin/apply">
                <input
                    type="hidden"
                    name=ADMIN_CSRF_FORM_FIELD
                    value=csrf_token
                />
                <div class="csv-header">{header}</div>
                <textarea name=name rows="10" cols="50">{body}</textarea>
                <button type="submit">"Apply"</button>
            </form>
        </section>
    }
}
