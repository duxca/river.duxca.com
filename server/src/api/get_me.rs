pub async fn get_me(
    pool: &sqlx::sqlite::SqlitePool,
    user_id: i64,
    model::api::get_me::Request {}: model::api::get_me::Request,
) -> Result<model::api::get_me::Response, anyhow::Error> {
    let user = crate::db::user::get_user(pool, user_id).await?;
    Ok(model::api::get_me::Response { user })
}
