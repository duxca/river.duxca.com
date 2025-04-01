#[tracing::instrument(level = "trace", skip(pool))]
pub async fn list_users(
    pool: &sqlx::sqlite::SqlitePool,
    _user: &model::user::User,
    model::api::list_users::Request { offset, limit }: model::api::list_users::Request,
) -> Result<model::api::list_users::Response, anyhow::Error> {
    let offset = offset.unwrap_or(0);
    let limit = limit.unwrap_or(20);
    let (users, next, total) = db::user::list_users(pool, offset, limit).await?;
    Ok(model::api::list_users::Response { users, next, total })
}
