pub async fn list_users(
    pool: &sqlx::sqlite::SqlitePool,
    model::api::list_users::Request { offset, limit }: model::api::list_users::Request,
) -> Result<model::api::list_users::Response, anyhow::Error> {
    let (users, next, total) = db::user::list_users(pool, offset, limit).await?;
    Ok(model::api::list_users::Response { users, next, total })
}
