#[tracing::instrument(level = "trace", skip(pool))]
pub async fn delete_river_track(
    pool: &sqlx::sqlite::SqlitePool,
    user: &model::user::User,
    model::api::delete_river_track::Request {
        river_track_id,
    }: model::api::delete_river_track::Request,
) -> Result<model::api::delete_river_track::Response, anyhow::Error> {
    let trk = db::river_tracks::get_river_track(pool, river_track_id).await?;
    if let Some(trk) = trk {
        if user.role == 0 {
            // 管理者は消せる
            db::river_tracks::delete_river_track(pool, river_track_id).await?;
        } else if trk.user_id == user.user_id {
            // 所有者かつ 24h 以内のみ消せる
            let now = std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;
            if now - trk.created_at < 24 * 60 * 60 {
                db::river_tracks::delete_river_track(pool, river_track_id).await?;
            }
        }
    }
    // TODO: error を返す
    Ok(model::api::delete_river_track::Response {})
}
