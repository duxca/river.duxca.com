pub async fn delete_river(
    pool: &sqlx::sqlite::SqlitePool,
    user: &model::user::User,
    model::api::delete_river::Request { river_id }: model::api::delete_river::Request,
) -> Result<model::api::delete_river::Response, anyhow::Error> {
    let rvr = db::rivers::get_river(pool, river_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("NotFound: river {river_id}"))?;

    if user.role == 0 {
        // 管理者は消せる
        db::rivers::delete_river(pool, river_id).await?;
        return Ok(model::api::delete_river::Response {});
    }

    if rvr.user_id != user.user_id {
        anyhow::bail!("PermissionDenied: river {river_id}");
    }

    // 所有者かつ 24h 以内のみ消せる
    let now = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    if now - rvr.created_at >= 24 * 60 * 60 {
        anyhow::bail!("Expired: river {river_id}");
    }

    db::rivers::delete_river(pool, river_id).await?;
    Ok(model::api::delete_river::Response {})
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

    async fn create_user(conn: &sqlx::SqlitePool, nickname: &str) -> Result<i64, anyhow::Error> {
        let row = sqlx::query!(
            r#"
            INSERT INTO users (nickname, role)
            VALUES (?1, 1)
            RETURNING user_id
            "#,
            nickname
        )
        .fetch_one(conn)
        .await?;
        Ok(row.user_id)
    }

    async fn backdate_river(
        conn: &sqlx::SqlitePool,
        river_id: i64,
        seconds: i64,
    ) -> Result<(), anyhow::Error> {
        let created_at = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)?
            .as_secs() as i64
            - seconds;
        sqlx::query!(
            r#"
            UPDATE rivers
            SET created_at = ?1
            WHERE river_id = ?2
            "#,
            created_at,
            river_id
        )
        .execute(conn)
        .await?;
        Ok(())
    }

    #[sqlx::test(migrations = "../db/migrations")]
    async fn delete_river_errors_and_allowed_deletes(
        conn: sqlx::SqlitePool,
    ) -> Result<(), anyhow::Error> {
        let owner_id = create_user(&conn, "owner").await?;
        let other_id = create_user(&conn, "other").await?;

        let missing = delete_river(
            &conn,
            &user(owner_id, 1),
            model::api::delete_river::Request { river_id: 404 },
        )
        .await
        .expect_err("missing river should error");
        assert!(missing.to_string().contains("NotFound"));

        let denied_river_id =
            db::rivers::create_river(&conn, owner_id, "denied", (35.0, 139.0), "").await?;
        let denied = delete_river(
            &conn,
            &user(other_id, 1),
            model::api::delete_river::Request {
                river_id: denied_river_id,
            },
        )
        .await
        .expect_err("non-owner should error");
        assert!(denied.to_string().contains("PermissionDenied"));
        assert!(
            db::rivers::get_river(&conn, denied_river_id)
                .await?
                .is_some()
        );

        let expired_river_id =
            db::rivers::create_river(&conn, owner_id, "expired", (35.0, 139.0), "").await?;
        backdate_river(&conn, expired_river_id, 25 * 60 * 60).await?;
        let expired = delete_river(
            &conn,
            &user(owner_id, 1),
            model::api::delete_river::Request {
                river_id: expired_river_id,
            },
        )
        .await
        .expect_err("expired owner delete should error");
        assert!(expired.to_string().contains("Expired"));
        assert!(
            db::rivers::get_river(&conn, expired_river_id)
                .await?
                .is_some()
        );

        let owner_river_id =
            db::rivers::create_river(&conn, owner_id, "owner", (35.0, 139.0), "").await?;
        delete_river(
            &conn,
            &user(owner_id, 1),
            model::api::delete_river::Request {
                river_id: owner_river_id,
            },
        )
        .await?;
        assert!(
            db::rivers::get_river(&conn, owner_river_id)
                .await?
                .is_none()
        );

        delete_river(
            &conn,
            &user(0, 0),
            model::api::delete_river::Request {
                river_id: expired_river_id,
            },
        )
        .await?;
        assert!(
            db::rivers::get_river(&conn, expired_river_id)
                .await?
                .is_none()
        );

        Ok(())
    }
}
