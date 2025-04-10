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
        // その identifier がおるか確認
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
                    user_auths.user_id = ?1
                AND user_auths.identity_type = ?2
                AND user_auths.identifier = ?3
                "#,
            user_id,
            identity_type,
            identifier
        )
        .fetch_all(&mut *tx)
        .await?;
        if let Some(ident) = identities.first() {
            // 既に登録済みの場合はそのまま返す
            let user = get_user(&mut *tx, ident.user_id).await?.unwrap();
            return Ok(user);
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
pub fn delete_user<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    user_id: i64,
) -> impl std::future::Future<Output = Result<(), anyhow::Error>> + Send + 'a {
    async move {
        let mut conn = conn.acquire().await?;
        sqlx::query!(
            r#"
            DELETE
            FROM users
            WHERE user_id = ?1
            "#,
            user_id
        )
        .execute(&mut *conn)
        .await?;
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
        Ok(())
    }
}
