pub mod create_river;
pub mod create_river_track;
pub mod create_river_waypoint;
pub mod delete_river;
pub mod delete_river_track;
pub mod delete_river_waypoint;
pub mod get_me;
pub mod get_river;
pub mod list_access_logs;
pub mod list_rivers;
pub mod list_users;

#[tracing::instrument(level = "trace", skip(db))]
pub async fn handler(
    db: &sqlx::sqlite::SqlitePool,
    user: &model::user::User,
    req: model::api::Request,
) -> Result<model::api::Response, anyhow::Error> {
    let mut conn = db.acquire().await?;
    if !req.check_permission(user) {
        return Ok(model::api::ErrorKind::PermissionDenied.into());
    }
    db::user::add_access_log(&mut *conn, user.user_id, &req).await?;
    match req {
        model::api::Request::GetMe(req) => {
            let res = crate::get_me::get_me(db, user, req).await?;
            Ok(res.into())
        }
        model::api::Request::ListUsers(req) => {
            let res = crate::list_users::list_users(db, user, req).await?;
            Ok(res.into())
        }
        model::api::Request::ListAccessLogs(req) => {
            let res = crate::list_access_logs::list_access_logs(db, user, req).await?;
            Ok(res.into())
        }
        model::api::Request::ListRivers(req) => {
            let res = crate::list_rivers::list_rivers(db, user, req).await?;
            Ok(res.into())
        }
        model::api::Request::GetRiver(req) => {
            let res = crate::get_river::get_river(db, user, req).await?;
            Ok(res.into())
        }
        model::api::Request::CreateRiver(req) => {
            let res = crate::create_river::create_river(db, user, req).await?;
            Ok(res.into())
        }
        model::api::Request::DeleteRiver(req) => {
            let res = crate::delete_river::delete_river(db, user, req).await?;
            Ok(res.into())
        }
        model::api::Request::CreateRiverWaypoint(req) => {
            let res = crate::create_river_waypoint::create_river_waypoint(db, user, req).await?;
            Ok(res.into())
        }
        model::api::Request::DeleteRiverWaypoint(req) => {
            let res = crate::delete_river_waypoint::delete_river_waypoint(db, user, req).await?;
            Ok(res.into())
        }
        model::api::Request::CreateRiverTrack(req) => {
            let res = crate::create_river_track::create_river_track(db, user, req).await?;
            Ok(res.into())
        }
        model::api::Request::DeleteRiverTrack(req) => {
            let res = crate::delete_river_track::delete_river_track(db, user, req).await?;
            Ok(res.into())
        }
    }
}
