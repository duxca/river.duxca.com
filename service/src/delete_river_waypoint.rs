pub async fn delete_river_waypoint(
    pool: &sqlx::sqlite::SqlitePool,
    user: model::user::User,
    model::api::delete_river_waypoint::Request {
        river_waypoint_id,
    }: model::api::delete_river_waypoint::Request,
) -> Result<model::api::delete_river_waypoint::Response, anyhow::Error> {
    let wpt = db::river_waypoints::get_river_waypoint(pool, river_waypoint_id).await?;
    if let Some(wpt) = wpt {
        // 所有者かつ 24h 以内のみ消せる
        if wpt.user_id == user.user_id {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;
            if now - wpt.created_at < 24 * 60 * 60 {
                db::river_waypoints::delete_river_waypoint(pool, river_waypoint_id).await?;
            }
        }
    }
    // TODO: error を返す
    Ok(model::api::delete_river_waypoint::Response {})
}
