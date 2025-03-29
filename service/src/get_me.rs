pub async fn get_me(
    _pool: &sqlx::sqlite::SqlitePool,
    user: model::user::User,
    model::api::get_me::Request {}: model::api::get_me::Request,
) -> Result<model::api::get_me::Response, anyhow::Error> {
    // 自分自身を返すだけ
    Ok(model::api::get_me::Response { user })
}
