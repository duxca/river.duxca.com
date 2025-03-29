#![allow(clippy::manual_async_fn)]

#[tracing::instrument(level = "trace", skip(conn))]
pub fn create_river_waypoint<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    river_id: i64,
    user_id: i64,
    waypoint_name: String,
    waypoint: (f64, f64),
    description: &'a str,
) -> impl std::future::Future<Output = Result<i64, anyhow::Error>> + Send + 'a {
    let waypoint = serde_json::json!(waypoint).to_string();
    async move {
        let mut conn = conn.acquire().await?;
        let row = sqlx::query!(
            r#"
            INSERT INTO river_waypoints (river_id, user_id, waypoint_name, description, waypoint)
            VALUES (?1, ?2, ?3, ?4, ?5)
            RETURNING river_waypoint_id;
            "#,
            river_id,
            user_id,
            waypoint_name,
            description,
            waypoint
        )
        .fetch_one(&mut *conn)
        .await?;
        let river_waypoint_id = row.river_waypoint_id;
        Ok(river_waypoint_id)
    }
}

#[tracing::instrument(level = "trace", skip(conn))]
pub fn list_river_waypoints_all<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    river_id: i64,
) -> impl std::future::Future<Output = Result<Vec<model::river::RiverWaypoint>, anyhow::Error>> + Send + 'a
{
    async move {
        let mut conn = conn.acquire().await?;
        let rows = sqlx::query_as!(
            model::river::RiverWaypoint,
            r#"
            SELECT
                river_waypoint_id,
                river_id,
                user_id,
                waypoint_name,
                description,
                waypoint AS "waypoint!: serde_json::Value",
                created_at,
                updated_at
            FROM river_waypoints
            WHERE river_id = ?1
            ORDER BY river_waypoint_id ASC
            "#,
            river_id
        )
        .fetch_all(&mut *conn)
        .await?;
        Ok(rows)
    }
}

#[tracing::instrument(level = "trace", skip(conn))]
pub fn update_river_waypoint<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    river_waypoint_id: i64,
    waypoint_name: String,
    waypoint: (f64, f64),
    description: Option<String>,
) -> impl std::future::Future<Output = Result<(), anyhow::Error>> + Send + 'a {
    let waypoint = serde_json::json!(waypoint).to_string();
    let description = description.unwrap_or_default();
    async move {
        let mut conn = conn.acquire().await?;
        sqlx::query!(
            r#"
            UPDATE river_waypoints
            SET waypoint_name = ?1, waypoint = ?2, description = ?3
            WHERE river_waypoint_id = ?4
            "#,
            waypoint_name,
            waypoint,
            description,
            river_waypoint_id
        )
        .execute(&mut *conn)
        .await?;
        Ok(())
    }
}

#[tracing::instrument(level = "trace", skip(conn))]
pub fn get_river_waypoint<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    river_waypoint_id: i64,
) -> impl std::future::Future<Output = Result<Option<model::river::RiverWaypoint>, anyhow::Error>>
+ Send
+ 'a {
    async move {
        let mut conn = conn.acquire().await?;
        let row = sqlx::query_as!(
            model::river::RiverWaypoint,
            r#"
            SELECT
                river_waypoint_id,
                river_id,
                user_id,
                waypoint_name,
                description,
                waypoint AS "waypoint!: serde_json::Value",
                created_at,
                updated_at
            FROM river_waypoints
            WHERE river_waypoint_id = ?1
            "#,
            river_waypoint_id
        )
        .fetch_optional(&mut *conn)
        .await?;
        Ok(row)
    }
}

#[tracing::instrument(level = "trace", skip(conn))]
pub fn delete_river_waypoint<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    river_waypoint_id: i64,
) -> impl std::future::Future<Output = Result<(), anyhow::Error>> + Send + 'a {
    async move {
        let mut conn = conn.acquire().await?;
        sqlx::query!(
            r#"
            DELETE FROM river_waypoints
            WHERE river_waypoint_id = ?1
            "#,
            river_waypoint_id
        )
        .execute(&mut *conn)
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test()]
    async fn river_waypoints_lifecycle_test(conn: sqlx::SqlitePool) -> Result<(), anyhow::Error> {
        env_logger::builder().is_test(true).try_init().ok();

        // テストユーザーを作成
        let row = sqlx::query!(
            r#"
            INSERT INTO users (nickname, role)
            VALUES ('test_user', 1)
            RETURNING user_id
            "#
        )
        .fetch_one(&conn)
        .await?;
        let user_id = row.user_id;

        // 川を作成
        let river_id =
            crate::rivers::create_river(&conn, user_id, "多摩川", (35.6435548, 139.7537994), "")
                .await?;

        // river_waypoint を追加
        let waypoint_id = create_river_waypoint(
            &conn,
            river_id,
            user_id,
            "二子玉川".to_string(),
            (35.6436000, 139.7538000),
            "テスト用の川のウェイポイント",
        )
        .await?;

        // 作成したwaypoint情報を取得して検証
        let waypoints = list_river_waypoints_all(&conn, river_id).await?;
        assert_eq!(waypoints.len(), 1);
        assert_eq!(waypoints[0].river_waypoint_id, waypoint_id);
        assert_eq!(waypoints[0].river_id, river_id);
        assert_eq!(waypoints[0].user_id, user_id);
        assert_eq!(waypoints[0].waypoint_name, "二子玉川");
        assert_eq!(waypoints[0].description, "テスト用の川のウェイポイント");

        // waypointを更新
        update_river_waypoint(
            &conn,
            waypoint_id,
            "二子玉川駅".to_string(),
            (35.6437000, 139.7539000),
            Some("更新後の説明".to_string()),
        )
        .await?;

        // 更新後のwaypoint情報を取得して検証
        let waypoints = list_river_waypoints_all(&conn, river_id).await?;
        assert_eq!(waypoints.len(), 1);
        assert_eq!(waypoints[0].river_waypoint_id, waypoint_id);
        assert_eq!(waypoints[0].waypoint_name, "二子玉川駅");
        assert_eq!(waypoints[0].description, "更新後の説明");

        Ok(())
    }
}
