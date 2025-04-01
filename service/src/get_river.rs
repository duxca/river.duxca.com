#[tracing::instrument(level = "trace", skip(pool))]
pub async fn get_river(
    pool: &sqlx::sqlite::SqlitePool,
    _user: &model::user::User,
    model::api::get_river::Request { river_id }: model::api::get_river::Request,
) -> Result<model::api::get_river::Response, anyhow::Error> {
    let river = db::rivers::get_river(pool, river_id).await?;
    let tracks = db::river_tracks::list_river_tracks_all(pool, river_id).await?;
    let waypoints = db::river_waypoints::list_river_waypoints_all(pool, river_id).await?;
    Ok(model::api::get_river::Response {
        river,
        tracks,
        waypoints,
    })
}
