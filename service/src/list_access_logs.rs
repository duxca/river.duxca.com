pub async fn list_access_logs(
    pool: &sqlx::sqlite::SqlitePool,
    _user: model::user::User,
    model::api::list_access_logs::Request {
        offset,
        limit,
        user_id,
    }: model::api::list_access_logs::Request,
) -> Result<model::api::list_access_logs::Response, anyhow::Error> {
    let offset = offset.unwrap_or(0);
    let limit = limit.unwrap_or(20);
    let (access_logs, next, total) =
        db::user::list_access_logs(pool, user_id, offset, limit).await?;
    Ok(model::api::list_access_logs::Response {
        access_logs,
        next,
        total,
    })
}
