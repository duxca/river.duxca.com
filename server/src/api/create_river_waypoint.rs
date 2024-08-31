pub async fn create_river_waypoint(
    pool: &sqlx::sqlite::SqlitePool,
    model::api::create_river_waypoint::Request {
        river_id,
        name,
        longitude,
        latitude,
    }: model::api::create_river_waypoint::Request,
) -> Result<model::api::create_river_waypoint::Response, anyhow::Error> {
    let river_waypoint_id =
        crate::db::river::create_river_waypoint(pool, river_id, name, longitude, latitude).await?;
    Ok(model::api::create_river_waypoint::Response { river_waypoint_id })
}
