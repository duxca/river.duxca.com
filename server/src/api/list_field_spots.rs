pub async fn list_field_spots(
    pool: &sqlx::sqlite::SqlitePool,
    model::api::list_field_spots::Request {
        offset,
        limit,
        river_id,
    }: model::api::list_field_spots::Request,
) -> Result<model::api::list_field_spots::Response, anyhow::Error> {
    let (river_waypoints, next, total) =
        crate::db::field::list_field_spots(pool, river_id, offset, limit).await?;
    Ok(model::api::list_field_spots::Response {
        river_waypoints,
        next,
        total,
    })
}
