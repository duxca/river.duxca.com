pub async fn create_river_track(
    pool: &sqlx::sqlite::SqlitePool,
    user: model::user::User,
    model::api::create_river_track::Request {
        river_id,
        track_name,
        description,
        track,
    }: model::api::create_river_track::Request,
) -> Result<model::api::create_river_track::Response, anyhow::Error> {
    let river_track_id = db::river_tracks::create_river_track(
        pool,
        river_id,
        user.user_id,
        &track_name,
        &track,
        &description,
    )
    .await?;
    Ok(model::api::create_river_track::Response { river_track_id })
}
