// pub mod create_river_waypoint;
pub mod get_river;
// pub mod list_access_logs;
pub mod list_rivers;
// pub mod list_users;

pub async fn handler(
    db: &sqlx::sqlite::SqlitePool,
    user: model::user::User,
    req: model::api::Request,
) -> Result<model::api::Response, anyhow::Error> {
    let mut conn = db.acquire().await?;
    if !req.check_permission(&user) {
        return Ok(model::api::ErrorKind::PermissionDenied.into());
    }
    db::user::add_access_log(&mut *conn, user.user_id, &req).await?;
    match req {
        model::api::Request::GetMe(..) => Ok(model::api::get_me::Response { user }.into()),
        // model::api::Request::ListUsers(req) => {
        //     let res = crate::list_users::list_users(db, req).await?;
        //     Ok(res.into())
        // }
        // model::api::Request::ListAccessLogs(req) => {
        //     let res = crate::list_access_logs::list_access_logs(db, req).await?;
        //     Ok(res.into())
        // }
        model::api::Request::ListRivers(req) => {
            let res = crate::list_rivers::list_rivers(db, req).await?;
            Ok(res.into())
        }
        model::api::Request::GetRiver(req) => {
            let res = crate::get_river::get_river(db, req).await?;
            let Some(res) = res else {
                return Ok(model::api::ErrorKind::NotFound.into());
            };
            Ok(res.into())
        } // model::api::Request::CreateRiverWaypoint(req) => {
          //     let res = crate::create_river_waypoint::create_river_waypoint(db, req).await?;
          //     Ok(res.into())
          // }
    }
}
