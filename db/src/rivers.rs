#![allow(clippy::manual_async_fn)]

#[tracing::instrument(level = "trace", skip(conn))]
pub fn create_river<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    river_name: &'a str,
    waypoint: (f64, f64),
) -> impl std::future::Future<Output = Result<i64, anyhow::Error>> + Send + 'a {
    let waypoint = serde_json::json!(waypoint).to_string();
    async move {
        let mut conn = conn.acquire().await?;
        let row = sqlx::query!(
            r#"
            INSERT INTO rivers (river_name, waypoint)
            VALUES (?1, ?2)
            RETURNING river_id;
            "#,
            river_name,
            waypoint
        )
        .fetch_one(&mut *conn)
        .await?;
        let river_id = row.river_id;
        Ok(river_id)
    }
}

#[tracing::instrument(level = "trace", skip(conn))]
pub fn list_rivers_all<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
) -> impl std::future::Future<Output = Result<Vec<model::river::River>, anyhow::Error>> + Send + 'a
{
    async move {
        let mut conn = conn.acquire().await?;
        let rows = sqlx::query_as!(
            model::river::River,
            r#"
            SELECT
                river_id,
                river_name,
                waypoint AS "waypoint!: serde_json::Value",
                created_at
            FROM rivers
            ORDER BY river_id ASC
            "#,
        )
        .fetch_all(&mut *conn)
        .await?;
        Ok(rows)
    }
}

#[tracing::instrument(level = "trace", skip(conn))]
pub fn update_river<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    river_id: i64,
    river_name: &'a str,
    waypoint: (f64, f64),
) -> impl std::future::Future<Output = Result<(), anyhow::Error>> + Send + 'a {
    let waypoint = serde_json::json!(waypoint).to_string();
    async move {
        let mut conn = conn.acquire().await?;
        sqlx::query!(
            r#"
            UPDATE rivers
            SET river_name = ?1, waypoint = ?2
            WHERE river_id = ?3
            "#,
            river_name,
            waypoint,
            river_id
        )
        .execute(&mut *conn)
        .await?;
        Ok(())
    }
}

#[allow(clippy::type_complexity)]
pub fn get_river<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    river_id: i64,
) -> impl std::future::Future<Output = Result<Option<model::river::River>, anyhow::Error>> + Send + 'a
{
    async move {
        let mut conn = conn.acquire().await?;
        let river = sqlx::query_as!(
            model::river::River,
            r#"
            SELECT
                river_id,
                river_name,
                waypoint AS "waypoint!: serde_json::Value",
                created_at
            FROM rivers
            WHERE river_id = ?1
            "#,
            river_id
        )
        .fetch_optional(&mut *conn)
        .await?;
        Ok(river)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test()]
    async fn river_lifecycle_test(conn: sqlx::SqlitePool) -> Result<(), anyhow::Error> {
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

        // 最初は川がn件であることを確認
        let rivers = list_rivers_all(&conn).await?;
        let first_river_count = rivers.len();

        // 川を作成
        let river_id = create_river(&conn, "多摩川", (35.6435548, 139.7537994)).await?;

        // 作成した川が取得できることを確認
        let river = get_river(&conn, river_id)
            .await?
            .expect("river should exist");
        assert_eq!(river.river_name, "多摩川");
        assert_eq!(river.waypoint, serde_json::json!([35.6435548, 139.7537994]));

        // 川の情報を更新
        update_river(&conn, river_id, "新・多摩川", (35.6435549, 139.7537995)).await?;

        // 更新後の情報を確認
        let updated_river = get_river(&conn, river_id)
            .await?
            .expect("river should exist");
        assert_eq!(updated_river.river_name, "新・多摩川");
        assert_eq!(
            updated_river.waypoint,
            serde_json::json!([35.6435549, 139.7537995])
        );

        // 川の一覧を確認
        let rivers = list_rivers_all(&conn).await?;
        assert_eq!(rivers.len(), first_river_count + 1);
        let last_river = rivers.last().expect("should have at least one river");
        assert_eq!(last_river.river_id, river_id);
        assert_eq!(last_river.river_name, "新・多摩川");

        Ok(())
    }
}
