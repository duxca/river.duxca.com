#[tracing::instrument(level = "trace", skip(pool))]
pub async fn list_rivers(
    pool: &sqlx::sqlite::SqlitePool,
    _user: &model::user::User,
    model::api::list_rivers::Request {}: model::api::list_rivers::Request,
) -> Result<model::api::list_rivers::Response, anyhow::Error> {
    let rivers = db::rivers::list_rivers_all(pool).await?;
    Ok(model::api::list_rivers::Response { rivers })
}
