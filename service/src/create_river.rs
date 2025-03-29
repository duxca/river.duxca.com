pub async fn create_river(
    pool: &sqlx::sqlite::SqlitePool,
    user: model::user::User,
    model::api::create_river::Request {
        name,
        latitude,
        longitude,
    }: model::api::create_river::Request,
) -> Result<model::api::create_river::Response, anyhow::Error> {
    let river_id =
        db::rivers::create_river(pool, user.user_id, &name, (latitude, longitude), "").await?;
    Ok(model::api::create_river::Response { river_id })
}
