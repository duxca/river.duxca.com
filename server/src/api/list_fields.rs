pub async fn list_fields(
    pool: &sqlx::sqlite::SqlitePool,
    model::api::list_fields::Request { offset, limit }: model::api::list_fields::Request,
) -> Result<model::api::list_fields::Response, anyhow::Error> {
    let (rivers, next, total) = crate::db::field::list_fields(pool, offset, limit).await?;
    Ok(model::api::list_fields::Response {
        rivers,
        next,
        total,
    })
}
