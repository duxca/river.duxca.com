#![allow(clippy::manual_async_fn)]

pub fn create_file<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    user_id: i64,
    content_type: &'a str,
    file_size: i64,
    gcs_path: &'a str,
) -> impl std::future::Future<Output = Result<i64, anyhow::Error>> + Send + 'a {
    async move {
        let mut conn = conn.acquire().await?;
        let row = sqlx::query!(
            r#"
            INSERT INTO files (user_id, content_type, file_size, gcs_path)
            VALUES (?, ?, ?, ?)
            RETURNING file_id
            "#,
            user_id,
            content_type,
            file_size,
            gcs_path
        )
        .fetch_one(&mut *conn)
        .await?;
        let file_id = row.file_id;
        Ok(file_id)
    }
}

pub fn get_file<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    file_id: i64,
) -> impl std::future::Future<Output = Result<Option<model::File>, anyhow::Error>> + Send + 'a {
    async move {
        let mut conn = conn.acquire().await?;
        let file = sqlx::query_as!(
            model::File,
            r#"
            SELECT
                file_id,
                user_id,
                content_type,
                file_size,
                gcs_path,
                created_at
            FROM files
            WHERE file_id = ?
            "#,
            file_id
        )
        .fetch_optional(&mut *conn)
        .await?;
        Ok(file)
    }
}

pub fn delete_file<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    file_id: i64,
) -> impl std::future::Future<Output = Result<(), anyhow::Error>> + Send + 'a {
    async move {
        let mut conn = conn.acquire().await?;
        let _res = sqlx::query!(
            r#"
            DELETE FROM files
            WHERE file_id = ?
            "#,
            file_id
        )
        .execute(&mut *conn)
        .await?;
        Ok(())
    }
}
