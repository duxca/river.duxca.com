// pub mod create_river_waypoint;
pub mod get_me;
pub mod list_access_logs;
pub mod list_field_spots;
pub mod list_fields;
pub mod list_users;

pub async fn handler(
    st: &crate::web::State,
    user_id: i64,
    req: model::api::Request,
) -> Result<model::api::Response, anyhow::Error> {
    let mut conn = st.db.acquire().await?;
    if !crate::db::user::check_permission(&mut *conn, user_id, &req).await? {
        return Ok(model::api::ErrorKind::PermissionDenied.into());
    }
    crate::db::user::add_access_log(&mut *conn, user_id, &req).await?;
    match req {
        model::api::Request::GetMe(req) => {
            let res = crate::api::get_me::get_me(&st.db, user_id, req).await?;
            Ok(res.into())
        }
        model::api::Request::ListUsers(req) => {
            let res = crate::api::list_users::list_users(&st.db, req).await?;
            Ok(res.into())
        }
        model::api::Request::ListAccessLogs(req) => {
            let res = crate::api::list_access_logs::list_access_logs(&st.db, req).await?;
            Ok(res.into())
        }
        model::api::Request::ListRivers(req) => {
            let res = crate::api::list_fields::list_fields(&st.db, req).await?;
            Ok(res.into())
        }
        model::api::Request::ListRiverWaypoints(req) => {
            let res = crate::api::list_field_spots::list_field_spots(&st.db, req).await?;
            Ok(res.into())
        }
        // model::api::Request::CreateRiverWaypoint(req) => {
        //     let res = crate::api::create_river_waypoint::create_river_waypoint(&st.db, req).await?;
        //     Ok(res.into())
        // }
    }
}
