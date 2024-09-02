pub async fn list_river_waypoints(
    pool: &sqlx::sqlite::SqlitePool,
    model::api::list_river_waypoints::Request {
        offset,
        limit,
        river_id,
    }: model::api::list_river_waypoints::Request,
) -> Result<model::api::list_river_waypoints::Response, anyhow::Error> {
    let (river_waypoints, next, total) =
        crate::db::river::list_river_waypoints(pool, river_id, offset, limit).await?;
    Ok(model::api::list_river_waypoints::Response {
        river_waypoints,
        next,
        total,
    })
}
