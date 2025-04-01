#[tracing::instrument(level = "trace", skip(pool))]
pub async fn create_river_waypoint(
    pool: &sqlx::sqlite::SqlitePool,
    user: &model::user::User,
    model::api::create_river_waypoint::Request {
        river_id,
        name,
        latitude,
        longitude,
    }: model::api::create_river_waypoint::Request,
) -> Result<model::api::create_river_waypoint::Response, anyhow::Error> {
    let river_waypoint_id = db::river_waypoints::create_river_waypoint(
        pool,
        river_id,
        user.user_id,
        name,
        (latitude, longitude),
        "",
    )
    .await?;
    Ok(model::api::create_river_waypoint::Response { river_waypoint_id })
}
