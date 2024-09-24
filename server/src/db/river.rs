// SEE: https://github.com/launchbadge/sqlx/issues/1635#issuecomment-1027791249
#![allow(clippy::manual_async_fn)]

#[tracing::instrument(level = "trace", skip(conn))]
pub fn list_rivers<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    offset: Option<i64>,
    limit: Option<i64>,
) -> impl std::future::Future<Output = Result<(Vec<model::river::River>, i64, i64), anyhow::Error>>
       + Send
       + 'a {
    async move {
        let mut conn = conn.acquire().await?;
        let limit = limit.unwrap_or(20);
        let offset = offset.unwrap_or(0);
        let rows = sqlx::query_as!(
            model::river::River,
            r#"
            SELECT
                river_id,
                name
            FROM rivers
            ORDER BY river_id ASC
            LIMIT ?1
            OFFSET ?2
            "#,
            limit,
            offset
        )
        .fetch_all(&mut *conn)
        .await?;
        let next = offset + rows.len() as i64;
        let row = sqlx::query!(
            r#"
            SELECT
                COUNT(*) AS total
            FROM rivers
            "#
        )
        .fetch_one(&mut *conn)
        .await?;
        Ok((rows, next, row.total))
    }
}

pub fn serach_river<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    river_name: String,
) -> impl std::future::Future<Output = Result<Option<model::river::River>, anyhow::Error>> + Send + 'a
{
    async move {
        let mut conn = conn.acquire().await?;
        let row = sqlx::query_as!(
            model::river::River,
            r#"
            SELECT
                river_id,
                name
            FROM rivers
            WHERE name LIKE ?1
            "#,
            river_name,
        )
        .fetch_optional(&mut *conn)
        .await?;
        Ok(row)
    }
}

#[tracing::instrument(level = "trace", skip(conn))]
pub fn list_river_waypoints<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    river_id: i64,
    offset: Option<i64>,
    limit: Option<i64>,
) -> impl std::future::Future<
    Output = Result<(Vec<model::river::RiverWaypoint>, i64, i64), anyhow::Error>,
> + Send
       + 'a {
    async move {
        let mut conn = conn.acquire().await?;
        let limit = limit.unwrap_or(20);
        let offset = offset.unwrap_or(0);
        let rows = sqlx::query_as!(
            model::river::RiverWaypoint,
            r#"
            SELECT
                river_id,
                river_waypoint_id,
                name,
                latitude,
                longitude
            FROM river_waypoints
            WHERE river_id = ?1
            ORDER BY river_waypoint_id ASC
            LIMIT ?2
            OFFSET ?3"#,
            river_id,
            limit,
            offset
        )
        .fetch_all(&mut *conn)
        .await?;
        let next = offset + rows.len() as i64;
        let row = sqlx::query!(
            r#"
            SELECT
                COUNT(*) AS total
            FROM river_waypoints
            WHERE river_id = ?1
            "#,
            river_id
        )
        .fetch_one(&mut *conn)
        .await?;
        Ok((rows, next, row.total))
    }
}

#[tracing::instrument(level = "trace", skip(conn))]
pub fn create_river_waypoint<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    river_id: i64,
    name: String,
    longitude: f64,
    latitude: f64,
) -> impl std::future::Future<Output = Result<i64, anyhow::Error>> + Send + 'a {
    async move {
        let mut conn = conn.acquire().await?;
        let row = sqlx::query!(
            r#"
            INSERT INTO river_waypoints (river_id, name, longitude, latitude)
            VALUES (?1, ?2, ?3, ?4)
            RETURNING river_waypoint_id;
            "#,
            river_id,
            name,
            longitude,
            latitude
        )
        .fetch_one(&mut *conn)
        .await?;
        let river_waypoint_id = row.river_waypoint_id;
        Ok(river_waypoint_id)
    }
}

#[tracing::instrument(level = "trace", skip(conn))]
pub fn upsert_river_waypoint<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    river_name: String,
    point_name: String,
    // 経度
    latitude: f64,
    // 緯度
    longitude: f64,
) -> impl std::future::Future<Output = Result<(), anyhow::Error>> + Send + 'a {
    async move {
        let mut conn = conn.acquire().await?;
        let river_id = {
            // if let Some(river) = serach_river(&mut *conn, river_name.clone()).await? {
            //     river.river_id
            // } else {
            let row = sqlx::query!(
                r#"
                INSERT INTO rivers (name)
                VALUES (?1)
                ON CONFLICT (name)
                DO UPDATE SET name = (?1)
                RETURNING river_id;
                "#,
                river_name
            )
            .fetch_one(&mut *conn)
            .await?;
            row.river_id
        };
        sqlx::query!(
            r#"
            INSERT INTO river_waypoints (river_id, name, longitude, latitude)
            VALUES (?1, ?2, ?3, ?4)
            ON CONFLICT (river_id, name, description, latitude, longitude)
            DO NOTHING;
            "#,
            river_id,
            point_name,
            longitude,
            latitude,
        )
        .execute(&mut *conn)
        .await?;
        Ok(())
    }
}
