// SEE: https://github.com/launchbadge/sqlx/issues/1635#issuecomment-1027791249
#![allow(clippy::manual_async_fn)]

#[tracing::instrument(level = "trace", skip(conn))]
pub fn list_users<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    offset: i64,
    limit: i64,
) -> impl std::future::Future<Output = Result<(Vec<model::user::User>, i64, i64), anyhow::Error>>
+ Send
+ 'a {
    async move {
        let mut conn = conn.acquire().await?;
        let rows = sqlx::query_as!(
            model::user::User,
            r#"
            SELECT
                user_id,
                nickname,
                role,
                created_at
            FROM users
            ORDER BY user_id ASC
            LIMIT ?1 OFFSET ?2
            "#,
            limit,
            offset
        )
        .fetch_all(&mut *conn)
        .await?;
        let next_offset = offset + rows.len() as i64;
        let row = sqlx::query!(
            r#"
            SELECT
                COUNT(*) AS total
            FROM users
            "#
        )
        .fetch_one(&mut *conn)
        .await?;
        Ok((rows, next_offset, row.total))
    }
}

// ログインのフロー
#[tracing::instrument(level = "trace", skip(conn))]
pub fn auth_or_create_user<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    identity_type: i64,
    identifier: &'a str,
    nickname: &'a str,
) -> impl std::future::Future<Output = Result<model::user::User, anyhow::Error>> + Send + 'a {
    async move {
        use sqlx::Connection;
        let mut conn = conn.acquire().await?;
        let mut tx = conn.begin().await?;
        let identities = sqlx::query_as!(
            model::user::UserAuth,
            r#"
            SELECT
                user_auth_id,
                user_id,
                identity_type,
                identifier,
                created_at
            FROM user_auths
            WHERE
                user_auths.identity_type = ?1
            AND user_auths.identifier = ?2
            "#,
            identity_type,
            identifier
        )
        .fetch_all(&mut *tx)
        .await?;
        if let Some(ident) = identities.first() {
            let user = get_user(&mut *tx, ident.user_id).await?.unwrap();
            // 既に登録済みの場合はそのまま返す
            return Ok(user);
        }
        // 新規登録
        let user = sqlx::query!(
            r#"
            INSERT INTO users (nickname)
            VALUES (?1)
            RETURNING user_id
            "#,
            nickname
        )
        .fetch_one(&mut *tx)
        .await?;
        // アカウント情報を登録
        sqlx::query!(
            r#"
            INSERT INTO user_auths (user_id, identity_type, identifier)
            VALUES (?1, ?2, ?3);
            "#,
            user.user_id,
            identity_type,
            identifier
        )
        .execute(&mut *tx)
        .await?;
        let user = get_user(&mut tx, user.user_id).await?.unwrap();
        tx.commit().await?;
        Ok(user)
    }
}

// ログイン済みのユーザーに対して、新しい認証情報を追加する
#[tracing::instrument(level = "trace", skip(conn))]
pub fn auth_or_add_user_auth<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    user_id: i64,
    // github: 0, facebook: 1, twitter: 2
    identity_type: i64,
    identifier: &'a str,
) -> impl std::future::Future<Output = Result<model::user::User, anyhow::Error>> + Send + 'a {
    async move {
        use sqlx::Connection;
        let mut conn = conn.acquire().await?;
        let mut tx = conn.begin().await?;
        // その provider identity が別ユーザーに紐づいていないか確認
        let identities = sqlx::query_as::<_, model::user::UserAuth>(
            r#"
                SELECT
                    user_auth_id,
                    user_id,
                    identity_type,
                    identifier,
                    created_at
                FROM user_auths
                WHERE
                    user_auths.identity_type = ?1
                AND user_auths.identifier = ?2
                "#,
        )
        .bind(identity_type)
        .bind(identifier)
        .fetch_all(&mut *tx)
        .await?;
        if let Some(ident) = identities.first() {
            if ident.user_id == user_id {
                let user = get_user(&mut *tx, ident.user_id).await?.unwrap();
                return Ok(user);
            }
            return Err(anyhow::anyhow!(
                "この認証情報は別のアカウントに連携済みです"
            ));
        }

        let existing_provider = sqlx::query_as::<_, model::user::UserAuth>(
            r#"
                SELECT
                    user_auth_id,
                    user_id,
                    identity_type,
                    identifier,
                    created_at
                FROM user_auths
                WHERE
                    user_auths.user_id = ?1
                AND user_auths.identity_type = ?2
                "#,
        )
        .bind(user_id)
        .bind(identity_type)
        .fetch_optional(&mut *tx)
        .await?;
        if existing_provider.is_some() {
            return Err(anyhow::anyhow!(
                "このアカウントには同じログイン方法が既に連携済みです"
            ));
        }
        sqlx::query!(
            r#"
                INSERT INTO user_auths (user_id, identity_type, identifier)
                VALUES (?1, ?2, ?3);
                "#,
            user_id,
            identity_type,
            identifier
        )
        .execute(&mut *tx)
        .await?;

        let user = get_user(&mut tx, user_id).await?.unwrap();
        tx.commit().await?;
        Ok(user)
    }
}

#[tracing::instrument(level = "trace", skip(conn))]
pub fn get_user<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    id: i64,
) -> impl std::future::Future<Output = Result<Option<model::user::User>, anyhow::Error>> + Send + 'a
{
    async move {
        let mut conn = conn.acquire().await?;
        let row = sqlx::query_as!(
            model::user::User,
            r#"
            SELECT
                user_id,
                nickname,
                role,
                created_at
            FROM users
            WHERE user_id = ?1
            "#,
            id
        )
        .fetch_optional(&mut *conn)
        .await?;
        Ok(row)
    }
}

#[tracing::instrument(level = "trace", skip(conn))]
pub fn list_user_auths<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    offset: i64,
    limit: i64,
) -> impl std::future::Future<Output = Result<(Vec<model::user::UserAuth>, i64), anyhow::Error>>
+ Send
+ 'a {
    async move {
        let mut conn = conn.acquire().await?;
        let rows = sqlx::query_as!(
            model::user::UserAuth,
            r#"
            SELECT
                user_auth_id,
                user_id,
                identity_type,
                identifier,
                created_at
            FROM user_auths
            ORDER BY user_auth_id ASC
            LIMIT ?1
            OFFSET ?2
            "#,
            limit,
            offset
        )
        .fetch_all(&mut *conn)
        .await?;
        let next_offset = offset + rows.len() as i64;
        Ok((rows, next_offset))
    }
}

#[tracing::instrument(level = "trace", skip(conn))]
pub fn get_user_auths<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    user_id: i64,
) -> impl std::future::Future<Output = Result<Vec<model::user::UserAuth>, anyhow::Error>> + Send + 'a
{
    async move {
        let mut conn = conn.acquire().await?;
        let rows = sqlx::query_as!(
            model::user::UserAuth,
            r#"
            SELECT
                user_auth_id,
                user_id,
                identity_type,
                identifier,
                created_at
            FROM user_auths
            WHERE user_auths.user_id = ?1
            "#,
            user_id
        )
        .fetch_all(&mut *conn)
        .await?;
        Ok(rows)
    }
}

#[tracing::instrument(level = "trace", skip(conn))]
pub fn get_user_delete_preview<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    user_id: i64,
) -> impl std::future::Future<Output = Result<model::user::UserDeletePreview, anyhow::Error>> + Send + 'a {
    async move {
        let mut conn = conn.acquire().await?;
        let river_count = sqlx::query!(
            r#"
            SELECT COUNT(*) AS count
            FROM rivers
            WHERE user_id = ?1
            "#,
            user_id
        )
        .fetch_one(&mut *conn)
        .await?
        .count;
        let track_count = sqlx::query!(
            r#"
            SELECT COUNT(*) AS count
            FROM river_tracks
            WHERE user_id = ?1
               OR river_id IN (SELECT river_id FROM rivers WHERE user_id = ?1)
            "#,
            user_id
        )
        .fetch_one(&mut *conn)
        .await?
        .count;
        let waypoint_count = sqlx::query!(
            r#"
            SELECT COUNT(*) AS count
            FROM river_waypoints
            WHERE user_id = ?1
               OR river_id IN (SELECT river_id FROM rivers WHERE user_id = ?1)
            "#,
            user_id
        )
        .fetch_one(&mut *conn)
        .await?
        .count;
        let auth_count = sqlx::query!(
            r#"
            SELECT COUNT(*) AS count
            FROM user_auths
            WHERE user_id = ?1
            "#,
            user_id
        )
        .fetch_one(&mut *conn)
        .await?
        .count;
        Ok(model::user::UserDeletePreview {
            river_count,
            track_count,
            waypoint_count,
            auth_count,
        })
    }
}

#[tracing::instrument(level = "trace", skip(conn))]
pub fn delete_user<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    user_id: i64,
) -> impl std::future::Future<Output = Result<(), anyhow::Error>> + Send + 'a {
    async move {
        use sqlx::Connection;

        let mut conn = conn.acquire().await?;
        let mut tx = conn.begin().await?;

        if get_user(&mut *tx, user_id).await?.is_none() {
            return Err(anyhow::anyhow!("User not found"));
        }

        let deleted_at = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)?
            .as_secs() as i64;

        sqlx::query!(
            r#"
            INSERT INTO deleted_access_logs (
                access_log_id,
                user_id,
                request,
                created_at,
                deleted_at
            )
            SELECT
                access_log_id,
                user_id,
                request,
                created_at,
                ?1
            FROM access_logs
            WHERE user_id = ?2
            "#,
            deleted_at,
            user_id
        )
        .execute(&mut *tx)
        .await?;

        sqlx::query!(
            r#"
            INSERT INTO deleted_user_auths (
                user_auth_id,
                user_id,
                identity_type,
                identifier,
                created_at,
                deleted_at
            )
            SELECT
                user_auth_id,
                user_id,
                identity_type,
                identifier,
                created_at,
                ?1
            FROM user_auths
            WHERE user_id = ?2
            "#,
            deleted_at,
            user_id
        )
        .execute(&mut *tx)
        .await?;

        sqlx::query!(
            r#"
            INSERT INTO deleted_river_tracks (
                river_track_id,
                river_id,
                user_id,
                track_name,
                description,
                track,
                created_at,
                updated_at,
                deleted_at
            )
            SELECT
                river_track_id,
                river_id,
                user_id,
                track_name,
                description,
                track,
                created_at,
                updated_at,
                ?1
            FROM river_tracks
            WHERE user_id = ?2
               OR river_id IN (SELECT river_id FROM rivers WHERE user_id = ?2)
            "#,
            deleted_at,
            user_id
        )
        .execute(&mut *tx)
        .await?;

        sqlx::query!(
            r#"
            INSERT INTO deleted_river_waypoints (
                river_waypoint_id,
                river_id,
                user_id,
                waypoint_name,
                description,
                waypoint,
                created_at,
                updated_at,
                deleted_at
            )
            SELECT
                river_waypoint_id,
                river_id,
                user_id,
                waypoint_name,
                description,
                waypoint,
                created_at,
                updated_at,
                ?1
            FROM river_waypoints
            WHERE user_id = ?2
               OR river_id IN (SELECT river_id FROM rivers WHERE user_id = ?2)
            "#,
            deleted_at,
            user_id
        )
        .execute(&mut *tx)
        .await?;

        sqlx::query!(
            r#"
            INSERT INTO deleted_rivers (
                river_id,
                user_id,
                river_name,
                waypoint,
                description,
                created_at,
                deleted_at
            )
            SELECT
                river_id,
                user_id,
                river_name,
                waypoint,
                description,
                created_at,
                ?1
            FROM rivers
            WHERE user_id = ?2
            "#,
            deleted_at,
            user_id
        )
        .execute(&mut *tx)
        .await?;

        sqlx::query!(
            r#"
            INSERT INTO deleted_users (
                user_id,
                nickname,
                role,
                created_at,
                deleted_at
            )
            SELECT
                user_id,
                nickname,
                role,
                created_at,
                ?1
            FROM users
            WHERE user_id = ?2
            "#,
            deleted_at,
            user_id
        )
        .execute(&mut *tx)
        .await?;

        sqlx::query!(
            r#"
            DELETE FROM river_tracks
            WHERE user_id = ?1
               OR river_id IN (SELECT river_id FROM rivers WHERE user_id = ?1)
            "#,
            user_id
        )
        .execute(&mut *tx)
        .await?;

        sqlx::query!(
            r#"
            DELETE FROM river_waypoints
            WHERE user_id = ?1
               OR river_id IN (SELECT river_id FROM rivers WHERE user_id = ?1)
            "#,
            user_id
        )
        .execute(&mut *tx)
        .await?;

        sqlx::query!(
            r#"
            DELETE FROM rivers
            WHERE user_id = ?1
            "#,
            user_id
        )
        .execute(&mut *tx)
        .await?;

        sqlx::query!(
            r#"
            DELETE FROM access_logs
            WHERE user_id = ?1
            "#,
            user_id
        )
        .execute(&mut *tx)
        .await?;

        sqlx::query!(
            r#"
            DELETE FROM user_auths
            WHERE user_id = ?1
            "#,
            user_id
        )
        .execute(&mut *tx)
        .await?;

        sqlx::query!(
            r#"
            DELETE FROM users
            WHERE user_id = ?1
            "#,
            user_id
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }
}

#[tracing::instrument(level = "trace", skip(conn))]
pub fn add_access_log<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    user_id: i64,
    req: &'a model::api::Request,
) -> impl std::future::Future<Output = Result<(), anyhow::Error>> + Send + 'a {
    async move {
        let mut conn = conn.acquire().await?;
        let request = serde_json::to_string(req)?;
        sqlx::query!(
            r#"
            INSERT INTO access_logs ( user_id, request )
            VALUES ( ?1, ?2 )
            "#,
            user_id,
            request
        )
        .execute(&mut *conn)
        .await?;
        Ok(())
    }
}

#[tracing::instrument(level = "trace", skip(conn))]
pub fn list_access_logs<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    // None の場合は全ユーザーのログを取得
    user_id: Option<i64>,
    offset: i64,
    limit: i64,
) -> impl std::future::Future<Output = Result<(Vec<model::user::AccessLog>, i64, i64), anyhow::Error>>
+ Send
+ 'a {
    async move {
        let mut conn = conn.acquire().await?;
        if let Some(user_id) = user_id {
            let rows = sqlx::query_as!(
                model::user::AccessLog,
                r#"
                SELECT
                    access_logs.access_log_id AS access_log_id,
                    access_logs.user_id AS user_id,
                    access_logs.request AS request,
                    access_logs.created_at AS created_at
                FROM access_logs
                WHERE access_logs.user_id = ?3
                ORDER BY access_log_id ASC
                LIMIT ?1 OFFSET ?2
                "#,
                limit,
                offset,
                user_id,
            )
            .fetch_all(&mut *conn)
            .await?;
            let next_offset = offset + rows.len() as i64;
            let row = sqlx::query!(
                r#"
                SELECT
                    COUNT(*) AS total
                FROM access_logs
                WHERE user_id = ?1
                "#,
                user_id
            )
            .fetch_one(&mut *conn)
            .await?;
            Ok((rows, next_offset, row.total))
        } else {
            let rows = sqlx::query_as!(
                model::user::AccessLog,
                r#"
                SELECT
                    access_logs.access_log_id AS access_log_id,
                    access_logs.user_id AS user_id,
                    access_logs.request AS request,
                    access_logs.created_at AS created_at
                FROM access_logs
                ORDER BY access_log_id ASC
                LIMIT ?1 OFFSET ?2
                "#,
                limit,
                offset
            )
            .fetch_all(&mut *conn)
            .await?;
            let next_offset = offset + rows.len() as i64;
            let row = sqlx::query!(
                r#"
                SELECT
                    COUNT(*) AS total
                FROM access_logs
                "#
            )
            .fetch_one(&mut *conn)
            .await?;
            Ok((rows, next_offset, row.total))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test()]
    async fn user_login_test(conn: sqlx::SqlitePool) -> Result<(), anyhow::Error> {
        env_logger::builder().is_test(true).try_init().ok();

        // first login with github
        let user = auth_or_create_user(&conn, 0, "github_id_2", "test_user").await?;
        assert_eq!(user.nickname, "test_user");
        assert_eq!(user.role, 1); // デフォルトはuser権限

        // second login with facebook - adding new auth to existing user
        let user_with_facebook =
            auth_or_add_user_auth(&conn, user.user_id, 1, "facebook_id_2").await?;
        assert_eq!(user_with_facebook.user_id, user.user_id);
        assert_eq!(user_with_facebook.nickname, "test_user");

        // check - attempt to login with facebook should return the same user
        let user_facebook_login =
            auth_or_create_user(&conn, 1, "facebook_id_2", "another_name").await?;
        assert_eq!(user_facebook_login.user_id, user.user_id);
        assert_eq!(user_facebook_login.nickname, "test_user");

        // show and check users join user_auths table
        let auths = sqlx::query_as!(
            model::user::UserAuth,
            r#"
            SELECT
                user_auth_id,
                user_id,
                identity_type,
                identifier,
                created_at
            FROM user_auths
            WHERE user_auths.user_id = ?1
            "#,
            user.user_id
        )
        .fetch_all(&conn)
        .await?;
        assert_eq!(auths.len(), 2);
        assert_eq!(auths[0].identity_type, 0);
        assert_eq!(auths[0].identifier, "github_id_2");
        assert_eq!(auths[1].identity_type, 1);
        assert_eq!(auths[1].identifier, "facebook_id_2");

        // check - attempt to login with github should return the same user
        let user = auth_or_create_user(&conn, 0, "github_id_2", "test_user").await?;
        assert_eq!(user.nickname, "test_user");
        assert_eq!(user.role, 1); // デフォルトはuser権限
        assert_eq!(user_facebook_login.user_id, user.user_id);

        let auths = sqlx::query_as!(
            model::user::UserAuth,
            r#"
            SELECT
                user_auth_id,
                user_id,
                identity_type,
                identifier,
                created_at
            FROM user_auths
            WHERE user_auths.user_id = ?1
            "#,
            user.user_id
        )
        .fetch_all(&conn)
        .await?;
        assert_eq!(auths.len(), 2);
        assert_eq!(auths[0].identity_type, 0);
        assert_eq!(auths[0].identifier, "github_id_2");
        assert_eq!(auths[1].identity_type, 1);
        assert_eq!(auths[1].identifier, "facebook_id_2");
        Ok(())
    }

    #[sqlx::test()]
    async fn user_lifecycle_test(conn: sqlx::SqlitePool) -> Result<(), anyhow::Error> {
        env_logger::builder().is_test(true).try_init().ok();
        // ユーザー作成のテスト
        let user1 = auth_or_create_user(&conn, 0, "github_id_1", "user1").await?;
        assert_eq!(user1.nickname, "user1");
        assert_eq!(user1.role, 1); // デフォルトはuser権限

        // ユーザー取得のテスト
        let user1_get = get_user(&conn, user1.user_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?;
        assert_eq!(user1_get, user1);

        // 存在しないユーザーの取得テスト
        let none_user = get_user(&conn, 9999).await?;
        assert!(none_user.is_none());

        // 同じidentity_typeとidentifierで再度作成を試みた場合は既存のユーザーが返される
        let user1_duplicate = auth_or_create_user(&conn, 0, "github_id_1", "user1_new").await?;
        assert_eq!(user1_duplicate, user1);

        // 別の認証情報を追加
        let user1_added_auth =
            auth_or_add_user_auth(&conn, user1.user_id, 1, "facebook_id_1").await?;
        assert_eq!(user1_added_auth, user1);

        // 2人目のユーザーを作成
        let user2 = auth_or_create_user(&conn, 2, "twitter_id_1", "user2").await?;
        assert_eq!(user2.nickname, "user2");

        // ユーザーリストのテスト
        let (users, next_offset, total) = list_users(&conn, 0, 10).await?;
        assert!(users.len() >= 2); // 初期データの admin + 作成した2ユーザー
        assert!(next_offset >= 2);
        assert!(total >= 2);

        // アクセスログのテスト
        let req = model::api::Request::GetMe(model::api::get_me::Request {});
        add_access_log(&conn, user1.user_id, &req).await?;

        // 特定ユーザーのアクセスログ取得
        let (logs, _, _) = list_access_logs(&conn, Some(user1.user_id), 0, 10).await?;
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].user_id, user1.user_id);

        // 全ユーザーのアクセスログ取得
        let (all_logs, _, _) = list_access_logs(&conn, None, 0, 10).await?;
        assert!(!all_logs.is_empty());

        // ユーザー削除のテスト
        delete_user(&conn, user2.user_id).await?;
        let deleted_user = get_user(&conn, user2.user_id).await?;
        assert!(deleted_user.is_none());

        let archived_user = sqlx::query!(
            r#"
            SELECT user_id, nickname, role
            FROM deleted_users
            WHERE user_id = ?1
            "#,
            user2.user_id
        )
        .fetch_optional(&conn)
        .await?;
        assert!(archived_user.is_some());

        let archived_auth = sqlx::query!(
            r#"
            SELECT identifier
            FROM deleted_user_auths
            WHERE user_id = ?1
            "#,
            user2.user_id
        )
        .fetch_optional(&conn)
        .await?;
        assert_eq!(archived_auth.map(|row| row.identifier), Some("twitter_id_1".to_string()));

        let reauth_user =
            auth_or_create_user(&conn, 2, "twitter_id_1", "user2_recreated").await?;
        assert_ne!(reauth_user.user_id, user2.user_id);
        assert_eq!(reauth_user.nickname, "user2_recreated");
        Ok(())
    }

    #[sqlx::test()]
    async fn user_archive_moves_related_data(conn: sqlx::SqlitePool) -> Result<(), anyhow::Error> {
        env_logger::builder().is_test(true).try_init().ok();

        let owner = auth_or_create_user(&conn, 0, "github_owner", "owner").await?;
        let contributor = auth_or_create_user(&conn, 1, "facebook_contributor", "contributor").await?;

        let river_id = crate::rivers::create_river(
            &conn,
            owner.user_id,
            "archive-test-river",
            (35.0, 139.0),
            "owner river",
        )
        .await?;
        crate::river_tracks::create_river_track(
            &conn,
            river_id,
            contributor.user_id,
            "contributor-track",
            &[(35.1, 139.1)],
            "track on owner river",
        )
        .await?;
        crate::river_tracks::create_river_track(
            &conn,
            river_id,
            contributor.user_id,
            "contributor-track-2",
            &[(35.2, 139.2)],
            "another track",
        )
        .await?;

        let other_river_id = crate::rivers::create_river(
            &conn,
            contributor.user_id,
            "contributor-river",
            (36.0, 140.0),
            "contributor river",
        )
        .await?;
        crate::river_tracks::create_river_track(
            &conn,
            other_river_id,
            contributor.user_id,
            "contributor-own-track",
            &[(36.1, 140.1)],
            "own track",
        )
        .await?;

        let req = model::api::Request::GetMe(model::api::get_me::Request {});
        add_access_log(&conn, contributor.user_id, &req).await?;

        delete_user(&conn, contributor.user_id).await?;

        assert!(get_user(&conn, contributor.user_id).await?.is_none());
        assert!(
            crate::rivers::get_river(&conn, other_river_id)
                .await?
                .is_none()
        );
        assert!(
            crate::rivers::get_river(&conn, river_id)
                .await?
                .is_some()
        );

        let archived_user = sqlx::query!(
            r#"
            SELECT nickname
            FROM deleted_users
            WHERE user_id = ?1
            "#,
            contributor.user_id
        )
        .fetch_one(&conn)
        .await?;
        assert_eq!(archived_user.nickname, "contributor");

        let archived_river_count = sqlx::query!(
            r#"
            SELECT COUNT(*) AS count
            FROM deleted_rivers
            WHERE user_id = ?1
            "#,
            contributor.user_id
        )
        .fetch_one(&conn)
        .await?;
        assert_eq!(archived_river_count.count, 1);

        let archived_track_count = sqlx::query!(
            r#"
            SELECT COUNT(*) AS count
            FROM deleted_river_tracks
            WHERE user_id = ?1
            "#,
            contributor.user_id
        )
        .fetch_one(&conn)
        .await?;
        assert_eq!(archived_track_count.count, 3);

        let archived_log_count = sqlx::query!(
            r#"
            SELECT COUNT(*) AS count
            FROM deleted_access_logs
            WHERE user_id = ?1
            "#,
            contributor.user_id
        )
        .fetch_one(&conn)
        .await?;
        assert_eq!(archived_log_count.count, 1);

        let active_track_count = sqlx::query!(
            r#"
            SELECT COUNT(*) AS count
            FROM river_tracks
            WHERE user_id = ?1
            "#,
            contributor.user_id
        )
        .fetch_one(&conn)
        .await?;
        assert_eq!(active_track_count.count, 0);

        let owner_river_tracks = sqlx::query!(
            r#"
            SELECT COUNT(*) AS count
            FROM river_tracks
            WHERE river_id = ?1
            "#,
            river_id
        )
        .fetch_one(&conn)
        .await?;
        assert_eq!(owner_river_tracks.count, 0);

        let recreated = auth_or_create_user(&conn, 1, "facebook_contributor", "new-contributor").await?;
        assert_ne!(recreated.user_id, contributor.user_id);
        assert_eq!(recreated.nickname, "new-contributor");
        Ok(())
    }

    #[sqlx::test()]
    async fn add_user_auth_rejects_conflicting_provider_identity(
        conn: sqlx::SqlitePool,
    ) -> Result<(), anyhow::Error> {
        let user1 = auth_or_create_user(&conn, 0, "github_id_1", "user1").await?;
        let user2 = auth_or_create_user(&conn, 1, "facebook_id_1", "user2").await?;

        let err = auth_or_add_user_auth(&conn, user2.user_id, 0, "github_id_1")
            .await
            .expect_err("provider identity linked to another user should be rejected");
        assert!(
            err.to_string()
                .contains("この認証情報は別のアカウントに連携済みです"),
            "{err:?}"
        );

        let user1_auths = get_user_auths(&conn, user1.user_id).await?;
        assert_eq!(user1_auths.len(), 1);
        let user2_auths = get_user_auths(&conn, user2.user_id).await?;
        assert_eq!(user2_auths.len(), 1);
        Ok(())
    }

    #[sqlx::test()]
    async fn add_user_auth_rejects_second_identity_for_same_provider(
        conn: sqlx::SqlitePool,
    ) -> Result<(), anyhow::Error> {
        let user = auth_or_create_user(&conn, 0, "github_id_1", "user1").await?;

        let err = auth_or_add_user_auth(&conn, user.user_id, 0, "github_id_2")
            .await
            .expect_err("a user should not connect two identities for the same provider");
        assert!(
            err.to_string()
                .contains("このアカウントには同じログイン方法が既に連携済みです"),
            "{err:?}"
        );

        let auths = get_user_auths(&conn, user.user_id).await?;
        assert_eq!(auths.len(), 1);
        assert_eq!(auths[0].identifier, "github_id_1");
        Ok(())
    }
}
