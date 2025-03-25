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
                river_name,
                waypoint AS "waypoint!: serde_json::Value",
                created_at
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

#[allow(clippy::type_complexity)]
pub fn get_river<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    river_id: i64,
) -> impl std::future::Future<
    Output = Result<
        (
            Option<model::river::River>,
            Vec<model::river::RiverTrack>,
            Vec<model::river::RiverWaypoint>,
        ),
        anyhow::Error,
    >,
> + Send
+ 'a {
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
        let waypoints = sqlx::query_as!(
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
        Ok((river, tracks, waypoints))
    }
}

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
pub fn create_river_track<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    river_id: i64,
    user_id: i64,
    track_name: &'a str,
    track: &'a [(f64, f64)],
    description: Option<&'a str>,
) -> impl std::future::Future<Output = Result<i64, anyhow::Error>> + Send + 'a {
    let track = serde_json::json!(track).to_string();
    let description = description.unwrap_or_default();
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

#[tracing::instrument(level = "trace", skip(conn))]
pub fn create_river_waypoint<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    river_id: i64,
    user_id: i64,
    waypoint_name: String,
    waypoint: (f64, f64),
    description: Option<String>,
) -> impl std::future::Future<Output = Result<i64, anyhow::Error>> + Send + 'a {
    let waypoint = serde_json::json!(waypoint).to_string();
    let description = description.unwrap_or_default();
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

pub fn get_river_track<'a, 'c>(
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

        // 最初は川が0件であることを確認
        let (rivers, next, total) = list_rivers(&conn, None, None).await?;
        assert!(rivers.is_empty());
        assert_eq!(next, 0);
        assert_eq!(total, 0);

        // 川を作成
        let river_id = create_river(&conn, "多摩川", (35.6435548, 139.7537994)).await?;

        // 川が1件取得できることを確認
        let (rivers, next, total) = list_rivers(&conn, None, None).await?;
        assert_eq!(rivers.len(), 1);
        assert_eq!(next, 1);
        assert_eq!(total, 1);
        assert_eq!(rivers[0].river_name, "多摩川");
        assert_eq!(
            serde_json::to_string(&rivers[0].waypoint).unwrap(),
            "[35.6435548,139.7537994]"
        );

        // river_track を追加
        let track_id = create_river_track(
            &conn,
            river_id,
            user_id,
            "多摩川上流",
            &[(35.6435548, 139.7537994), (35.6436000, 139.7538000)],
            Some("テスト用の川の軌跡"),
        )
        .await?;

        // river_track を取得して確認
        let tracks = get_river_track(&conn, river_id).await?;
        assert_eq!(tracks.len(), 1);
        assert_eq!(tracks[0].river_track_id, track_id);
        assert_eq!(tracks[0].track_name, "多摩川上流");
        assert_eq!(tracks[0].description, "テスト用の川の軌跡");
        let track: Vec<(f64, f64)> = serde_json::from_value(tracks[0].track.clone())?;
        assert_eq!(
            track,
            vec![(35.6435548, 139.7537994), (35.6436000, 139.7538000)]
        );

        // river_waypoint を追加
        let waypoint_id = create_river_waypoint(
            &conn,
            river_id,
            user_id,
            "二子玉川".to_string(),
            (35.6436000, 139.7538000),
            Some("テスト用の川のウェイポイント".to_string()),
        )
        .await?;

        // river_waypoint を含めて川の情報を取得して確認
        let (river, tracks, waypoints) = get_river(&conn, river_id).await?;
        let river = river.unwrap();
        assert_eq!(river.river_id, river_id);
        assert_eq!(river.river_name, "多摩川");
        assert_eq!(
            serde_json::to_string(&river.waypoint).unwrap(),
            "[35.6435548,139.7537994]"
        );
        assert_eq!(tracks.len(), 1);
        assert_eq!(waypoints.len(), 1);
        assert_eq!(waypoints[0].river_waypoint_id, waypoint_id);
        assert_eq!(waypoints[0].waypoint_name, "二子玉川");
        assert_eq!(waypoints[0].description, "テスト用の川のウェイポイント");
        assert_eq!(
            serde_json::to_string(&waypoints[0].waypoint).unwrap(),
            "[35.6436,139.7538]"
        );

        Ok(())
    }
}
