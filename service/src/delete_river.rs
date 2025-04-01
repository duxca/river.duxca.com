pub async fn delete_river(
    pool: &sqlx::sqlite::SqlitePool,
    user: &model::user::User,
    model::api::delete_river::Request { river_id }: model::api::delete_river::Request,
) -> Result<model::api::delete_river::Response, anyhow::Error> {
    let rvr = db::rivers::get_river(pool, river_id).await?;
    if let Some(rvr) = rvr {
        if user.role == 0 {
            // 管理者は消せる
            db::rivers::delete_river(pool, river_id).await?;
        } else if rvr.user_id == user.user_id {
            // 所有者かつ 24h 以内のみ消せる
            let now = std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;
            if now - rvr.created_at < 24 * 60 * 60 {
                db::rivers::delete_river(pool, river_id).await?;
            }
        }
    }
    // TODO: error を返す
    Ok(model::api::delete_river::Response {})
}
