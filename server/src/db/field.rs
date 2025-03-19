// SEE: https://github.com/launchbadge/sqlx/issues/1635#issuecomment-1027791249
#![allow(clippy::manual_async_fn)]

#[tracing::instrument(level = "trace", skip(conn))]
pub fn list_fields<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    offset: Option<i64>,
    limit: Option<i64>,
) -> impl std::future::Future<Output = Result<(Vec<model::field::Field>, i64, i64), anyhow::Error>>
       + Send
       + 'a {
    async move {
        let mut conn = conn.acquire().await?;
        let limit = limit.unwrap_or(20);
        let offset = offset.unwrap_or(0);
        let rows = sqlx::query_as!(
            model::field::Field,
            r#"
            SELECT
                field_id,
                field_name,
                description,
                route,
                created_at,
                updated_at
            FROM fields
            ORDER BY field_id ASC
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
            FROM fields
            "#
        )
        .fetch_one(&mut *conn)
        .await?;
        Ok((rows, next, row.total))
    }
}

pub fn search_field<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    field_name: String,
) -> impl std::future::Future<Output = Result<Option<model::field::Field>, anyhow::Error>> + Send + 'a
{
    async move {
        let mut conn = conn.acquire().await?;
        let row = sqlx::query_as!(
            model::field::Field,
            r#"
            SELECT
                field_id,
                field_name,
                description,
                route,
                created_at,
                updated_at
            FROM fields
            WHERE field_name LIKE ?1
            "#,
            field_name,
        )
        .fetch_optional(&mut *conn)
        .await?;
        Ok(row)
    }
}

#[tracing::instrument(level = "trace", skip(conn))]
pub fn list_field_spots<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    field_id: i64,
    offset: Option<i64>,
    limit: Option<i64>,
) -> impl std::future::Future<
    Output = Result<(Vec<model::field::FieldSpot>, i64, i64), anyhow::Error>,
> + Send
       + 'a {
    async move {
        let mut conn = conn.acquire().await?;
        let limit = limit.unwrap_or(20);
        let offset = offset.unwrap_or(0);
        let rows = sqlx::query_as!(
            model::field::FieldSpot,
            r#"
            SELECT
                field_spot_id,
                field_id,
                spot_name,
                spot_type,
                description,
                latitude,
                longitude,
                created_at,
                updated_at
            FROM field_spots
            WHERE field_id = ?1
            ORDER BY field_spot_id ASC
            LIMIT ?2
            OFFSET ?3"#,
            field_id,
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
            FROM field_spots
            WHERE field_id = ?1
            "#,
            field_id
        )
        .fetch_one(&mut *conn)
        .await?;
        Ok((rows, next, row.total))
    }
}

#[tracing::instrument(level = "trace", skip(conn))]
pub fn create_field_spot<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    field_id: i64,
    spot_name: String,
    longitude: f64,
    latitude: f64,
) -> impl std::future::Future<Output = Result<i64, anyhow::Error>> + Send + 'a {
    async move {
        let mut conn = conn.acquire().await?;
        let row = sqlx::query!(
            r#"
            INSERT INTO field_spots (field_id, spot_name, longitude, latitude)
            VALUES (?1, ?2, ?3, ?4)
            RETURNING field_spot_id;
            "#,
            field_id,
            spot_name,
            longitude,
            latitude
        )
        .fetch_one(&mut *conn)
        .await?;
        let field_spot_id = row.field_spot_id;
        Ok(field_spot_id)
    }
}

#[tracing::instrument(level = "trace", skip(conn))]
pub fn upsert_field_spot<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    field_name: String,
    spot_name: String,
    // 経度
    latitude: f64,
    // 緯度
    longitude: f64,
) -> impl std::future::Future<Output = Result<(), anyhow::Error>> + Send + 'a {
    async move {
        let mut conn = conn.acquire().await?;
        let field_id = {
            // upserting
            let row = sqlx::query!(
                r#"
                INSERT INTO fields (field_name)
                VALUES (?1)
                ON CONFLICT (field_name)
                DO UPDATE SET field_name = (?1)
                RETURNING field_id;
                "#,
                field_name
            )
            .fetch_one(&mut *conn)
            .await?;
            row.field_id
        };
        sqlx::query!(
            r#"
            INSERT INTO field_spots (field_id, spot_name, longitude, latitude)
            VALUES (?1, ?2, ?3, ?4)
            ON CONFLICT (field_id, spot_name, latitude, longitude)
            DO NOTHING;
            "#,
            field_id,
            spot_name,
            longitude,
            latitude,
        )
        .execute(&mut *conn)
        .await?;
        Ok(())
    }
}
