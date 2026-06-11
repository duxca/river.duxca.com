pub async fn delete_me(
    pool: &sqlx::sqlite::SqlitePool,
    user: &model::user::User,
    model::api::delete_me::Request {}: model::api::delete_me::Request,
) -> Result<Result<model::api::delete_me::Response, model::api::ErrorKind>, anyhow::Error> {
    if user.role == 0 {
        return Ok(Err(model::api::ErrorKind::PermissionDenied));
    }

    db::user::delete_user(pool, user.user_id).await?;
    Ok(Ok(model::api::delete_me::Response {}))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn user(user_id: i64, role: i64) -> model::user::User {
        model::user::User {
            user_id,
            nickname: format!("user-{user_id}"),
            role,
            created_at: 0,
        }
    }

    #[sqlx::test(migrations = "../db/migrations")]
    async fn delete_me_archives_user_and_allows_reauth(
        conn: sqlx::SqlitePool,
    ) -> Result<(), anyhow::Error> {
        let deleted =
            db::user::auth_or_create_user(&conn, 0, "github_delete_me", "delete-me").await?;
        let res = delete_me(&conn, &deleted, model::api::delete_me::Request {}).await?;
        assert!(res.is_ok());
        assert!(db::user::get_user(&conn, deleted.user_id).await?.is_none());

        let archived = sqlx::query!(
            r#"
            SELECT nickname
            FROM deleted_users
            WHERE user_id = ?1
            "#,
            deleted.user_id
        )
        .fetch_optional(&conn)
        .await?;
        assert_eq!(
            archived.map(|row| row.nickname),
            Some("delete-me".to_string())
        );

        let recreated =
            db::user::auth_or_create_user(&conn, 0, "github_delete_me", "delete-me-again").await?;
        assert_ne!(recreated.user_id, deleted.user_id);
        Ok(())
    }

    #[sqlx::test(migrations = "../db/migrations")]
    async fn delete_me_rejects_admin(conn: sqlx::SqlitePool) -> Result<(), anyhow::Error> {
        let res = delete_me(&conn, &user(1, 0), model::api::delete_me::Request {}).await?;
        assert_eq!(res, Err(model::api::ErrorKind::PermissionDenied));
        Ok(())
    }
}
