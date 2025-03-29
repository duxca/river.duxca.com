#![allow(clippy::manual_async_fn)]

#[tracing::instrument(level = "trace", skip(conn))]
pub fn create_river_track<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    river_id: i64,
    user_id: i64,
    track_name: &'a str,
    track: &'a [(f64, f64)],
    description: &'a str,
) -> impl std::future::Future<Output = Result<i64, anyhow::Error>> + Send + 'a {
    let track = serde_json::json!(track).to_string();
    async move {
        let mut conn = conn.acquire().await?;
        let row = sqlx::query!(
            r#"
            INSERT INTO river_tracks (river_id, user_id, track_name, track, description)
            VALUES (?1, ?2, ?3, ?4, ?5)
            RETURNING river_track_id;
            "#,
            river_id,
            user_id,
            track_name,
            track,
            description
        )
        .fetch_one(&mut *conn)
        .await?;
        let river_track_id = row.river_track_id;
        Ok(river_track_id)
    }
}

pub fn list_river_tracks_all<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    river_id: i64,
) -> impl std::future::Future<Output = Result<Vec<model::river::RiverTrack>, anyhow::Error>> + Send + 'a
{
    async move {
        let mut conn = conn.acquire().await?;
        let tracks = sqlx::query_as!(
            model::river::RiverTrack,
            r#"
            SELECT
                river_track_id,
                river_id,
                user_id,
                track_name,
                description,
                track AS "track!: serde_json::Value",
                created_at,
                updated_at
            FROM river_tracks
            WHERE river_id = ?1
            ORDER BY river_track_id ASC
            "#,
            river_id
        )
        .fetch_all(&mut *conn)
        .await?;
        Ok(tracks)
    }
}

#[tracing::instrument(level = "trace", skip(conn))]
pub fn update_river_track<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    river_track_id: i64,
    track_name: &'a str,
    track: &'a [(f64, f64)],
    description: Option<&'a str>,
) -> impl std::future::Future<Output = Result<(), anyhow::Error>> + Send + 'a {
    let track = serde_json::json!(track).to_string();
    let description = description.unwrap_or_default();
    async move {
        let mut conn = conn.acquire().await?;
        sqlx::query!(
            r#"
            UPDATE river_tracks
            SET track_name = ?1, track = ?2, description = ?3
            WHERE river_track_id = ?4
            "#,
            track_name,
            track,
            description,
            river_track_id
        )
        .execute(&mut *conn)
        .await?;
        Ok(())
    }
}

#[tracing::instrument(level = "trace", skip(conn))]
pub fn get_river_track<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    river_track_id: i64,
) -> impl std::future::Future<Output = Result<Option<model::river::RiverTrack>, anyhow::Error>> + Send + 'a
{
    async move {
        let mut conn = conn.acquire().await?;
        let row = sqlx::query_as!(
            model::river::RiverTrack,
            r#"
            SELECT
                river_track_id,
                river_id,
                user_id,
                track_name,
                description,
                track AS "track!: serde_json::Value",
                created_at,
                updated_at
            FROM river_tracks
            WHERE river_track_id = ?1
            "#,
            river_track_id
        )
        .fetch_optional(&mut *conn)
        .await?;
        Ok(row)
    }
}

#[tracing::instrument(level = "trace", skip(conn))]
pub fn delete_river_track<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    river_track_id: i64,
) -> impl std::future::Future<Output = Result<(), anyhow::Error>> + Send + 'a {
    async move {
        let mut conn = conn.acquire().await?;
        sqlx::query!(
            r#"
            DELETE FROM river_tracks
            WHERE river_track_id = ?1
            "#,
            river_track_id
        )
        .execute(&mut *conn)
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[sqlx::test()]
    async fn river_tracks_lifecycle_test(conn: sqlx::SqlitePool) -> Result<(), anyhow::Error> {
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
            crate::rivers::create_river(&conn, 0, "多摩川", (35.6435548, 139.7537994), "").await?;

        // river_track を追加
        let track_id = crate::river_tracks::create_river_track(
            &conn,
            river_id,
            user_id,
            "多摩川上流",
            &[(35.6435548, 139.7537994), (35.6436000, 139.7538000)],
            "テスト用の川の軌跡",
        )
        .await?;

        // 作成した river_track を取得して検証
        let tracks = crate::river_tracks::list_river_tracks_all(&conn, river_id).await?;
        assert_eq!(tracks.len(), 1);
        let track = &tracks[0];
        assert_eq!(track.river_track_id, track_id);
        assert_eq!(track.river_id, river_id);
        assert_eq!(track.user_id, user_id);
        assert_eq!(track.track_name, "多摩川上流");
        assert_eq!(track.description, "テスト用の川の軌跡");
        let track_points: Vec<(f64, f64)> = serde_json::from_value(track.track.clone())?;
        assert_eq!(
            track_points,
            vec![(35.6435548, 139.7537994), (35.6436000, 139.7538000)]
        );

        // river_track を更新
        crate::river_tracks::update_river_track(
            &conn,
            track_id,
            "多摩川上流（更新）",
            &[
                (35.6435548, 139.7537994),
                (35.6436000, 139.7538000),
                (35.6437000, 139.7539000),
            ],
            Some("更新後のテスト用の川の軌跡"),
        )
        .await?;

        // 更新後の river_track を取得して検証
        let tracks = crate::river_tracks::list_river_tracks_all(&conn, river_id).await?;
        assert_eq!(tracks.len(), 1);
        let track = &tracks[0];
        assert_eq!(track.river_track_id, track_id);
        assert_eq!(track.track_name, "多摩川上流（更新）");
        assert_eq!(track.description, "更新後のテスト用の川の軌跡");
        let track_points: Vec<(f64, f64)> = serde_json::from_value(track.track.clone())?;
        assert_eq!(
            track_points,
            vec![
                (35.6435548, 139.7537994),
                (35.6436000, 139.7538000),
                (35.6437000, 139.7539000)
            ]
        );

        Ok(())
    }
}
