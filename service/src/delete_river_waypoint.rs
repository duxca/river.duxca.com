#[tracing::instrument(level = "trace", skip(pool))]
pub async fn delete_river_waypoint(
    pool: &sqlx::sqlite::SqlitePool,
    user: &model::user::User,
    model::api::delete_river_waypoint::Request {
        river_waypoint_id,
    }: model::api::delete_river_waypoint::Request,
) -> Result<Result<model::api::delete_river_waypoint::Response, model::api::ErrorKind>, anyhow::Error>
{
    let Some(wpt) = db::river_waypoints::get_river_waypoint(pool, river_waypoint_id).await? else {
        return Ok(Err(model::api::ErrorKind::NotFound));
    };

    if user.role == 0 {
        // 管理者は消せる
        db::river_waypoints::delete_river_waypoint(pool, river_waypoint_id).await?;
        return Ok(Ok(model::api::delete_river_waypoint::Response {}));
    }

    if wpt.user_id != user.user_id {
        return Ok(Err(model::api::ErrorKind::PermissionDenied));
    }

    // 所有者かつ 24h 以内のみ消せる
    let now = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    if now - wpt.created_at >= 24 * 60 * 60 {
        return Ok(Err(model::api::ErrorKind::Expired));
    }

    db::river_waypoints::delete_river_waypoint(pool, river_waypoint_id).await?;
    Ok(Ok(model::api::delete_river_waypoint::Response {}))
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

    async fn create_waypoint(
        conn: &sqlx::SqlitePool,
        user_id: i64,
        name: &str,
    ) -> Result<i64, anyhow::Error> {
        let river_id = db::rivers::create_river(conn, user_id, name, (35.0, 139.0), "").await?;
        db::river_waypoints::create_river_waypoint(
            conn,
            river_id,
            user_id,
            name.to_string(),
            (35.0, 139.0),
            "",
        )
        .await
    }

    async fn backdate_waypoint(
        conn: &sqlx::SqlitePool,
        river_waypoint_id: i64,
        seconds: i64,
    ) -> Result<(), anyhow::Error> {
        let created_at = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)?
            .as_secs() as i64
            - seconds;
        sqlx::query!(
            r#"
            UPDATE river_waypoints
            SET created_at = ?1
            WHERE river_waypoint_id = ?2
            "#,
            created_at,
            river_waypoint_id
        )
        .execute(conn)
        .await?;
        Ok(())
    }

    #[sqlx::test(migrations = "../db/migrations")]
    async fn delete_river_waypoint_errors_and_allowed_deletes(
        conn: sqlx::SqlitePool,
    ) -> Result<(), anyhow::Error> {
        let owner_id = create_user(&conn, "owner").await?;
        let other_id = create_user(&conn, "other").await?;

        let missing = delete_river_waypoint(
            &conn,
            &user(owner_id, 1),
            model::api::delete_river_waypoint::Request {
                river_waypoint_id: 404,
            },
        )
        .await?;
        assert_eq!(missing, Err(model::api::ErrorKind::NotFound));

        let denied_waypoint_id = create_waypoint(&conn, owner_id, "denied").await?;
        let denied = delete_river_waypoint(
            &conn,
            &user(other_id, 1),
            model::api::delete_river_waypoint::Request {
                river_waypoint_id: denied_waypoint_id,
            },
        )
        .await?;
        assert_eq!(denied, Err(model::api::ErrorKind::PermissionDenied));
        assert!(
            db::river_waypoints::get_river_waypoint(&conn, denied_waypoint_id)
                .await?
                .is_some()
        );

        let expired_waypoint_id = create_waypoint(&conn, owner_id, "expired").await?;
        backdate_waypoint(&conn, expired_waypoint_id, 25 * 60 * 60).await?;
        let expired = delete_river_waypoint(
            &conn,
            &user(owner_id, 1),
            model::api::delete_river_waypoint::Request {
                river_waypoint_id: expired_waypoint_id,
            },
        )
        .await?;
        assert_eq!(expired, Err(model::api::ErrorKind::Expired));
        assert!(
            db::river_waypoints::get_river_waypoint(&conn, expired_waypoint_id)
                .await?
                .is_some()
        );

        let owner_waypoint_id = create_waypoint(&conn, owner_id, "owner").await?;
        let owner_delete = delete_river_waypoint(
            &conn,
            &user(owner_id, 1),
            model::api::delete_river_waypoint::Request {
                river_waypoint_id: owner_waypoint_id,
            },
        )
        .await?;
        assert!(owner_delete.is_ok());
        assert!(
            db::river_waypoints::get_river_waypoint(&conn, owner_waypoint_id)
                .await?
                .is_none()
        );

        let admin_delete = delete_river_waypoint(
            &conn,
            &user(0, 0),
            model::api::delete_river_waypoint::Request {
                river_waypoint_id: expired_waypoint_id,
            },
        )
        .await?;
        assert!(admin_delete.is_ok());
        assert!(
            db::river_waypoints::get_river_waypoint(&conn, expired_waypoint_id)
                .await?
                .is_none()
        );

        Ok(())
    }
}
