pub async fn delete_river(
    pool: &sqlx::sqlite::SqlitePool,
    user: &model::user::User,
    model::api::delete_river::Request { river_id }: model::api::delete_river::Request,
) -> Result<Result<model::api::delete_river::Response, model::api::ErrorKind>, anyhow::Error> {
    let Some(rvr) = db::rivers::get_river(pool, river_id).await? else {
        return Ok(Err(model::api::ErrorKind::NotFound));
    };

    if user.role == 0 {
        // 管理者は消せる
        db::rivers::delete_river(pool, river_id).await?;
        return Ok(Ok(model::api::delete_river::Response {}));
    }

    if rvr.user_id != user.user_id {
        return Ok(Err(model::api::ErrorKind::PermissionDenied));
    }

    // 所有者かつ 24h 以内のみ消せる
    let now = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    if now - rvr.created_at >= 24 * 60 * 60 {
        return Ok(Err(model::api::ErrorKind::Expired));
    }

    db::rivers::delete_river(pool, river_id).await?;
    Ok(Ok(model::api::delete_river::Response {}))
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
        .await?;
        assert_eq!(missing, Err(model::api::ErrorKind::NotFound));

        let denied_river_id =
            db::rivers::create_river(&conn, owner_id, "denied", (35.0, 139.0), "").await?;
        let denied = delete_river(
            &conn,
            &user(other_id, 1),
            model::api::delete_river::Request {
                river_id: denied_river_id,
            },
        )
        .await?;
        assert_eq!(denied, Err(model::api::ErrorKind::PermissionDenied));
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
        .await?;
        assert_eq!(expired, Err(model::api::ErrorKind::Expired));
        assert!(
            db::rivers::get_river(&conn, expired_river_id)
                .await?
                .is_some()
        );

        let owner_river_id =
            db::rivers::create_river(&conn, owner_id, "owner", (35.0, 139.0), "").await?;
        let owner_delete = delete_river(
            &conn,
            &user(owner_id, 1),
            model::api::delete_river::Request {
                river_id: owner_river_id,
            },
        )
        .await?;
        assert!(owner_delete.is_ok());
        assert!(
            db::rivers::get_river(&conn, owner_river_id)
                .await?
                .is_none()
        );

        let admin_delete = delete_river(
            &conn,
            &user(0, 0),
            model::api::delete_river::Request {
                river_id: expired_river_id,
            },
        )
        .await?;
        assert!(admin_delete.is_ok());
        assert!(
            db::rivers::get_river(&conn, expired_river_id)
                .await?
                .is_none()
        );

        Ok(())
    }
}
