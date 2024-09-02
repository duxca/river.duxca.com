pub async fn list_rivers(
    pool: &sqlx::sqlite::SqlitePool,
    model::api::list_rivers::Request { offset, limit }: model::api::list_rivers::Request,
) -> Result<model::api::list_rivers::Response, anyhow::Error> {
    let (rivers, next, total) = crate::db::river::list_rivers(pool, offset, limit).await?;
    Ok(model::api::list_rivers::Response {
        rivers,
        next,
        total
    })
}
