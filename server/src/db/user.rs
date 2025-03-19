// SEE: https://github.com/launchbadge/sqlx/issues/1635#issuecomment-1027791249
#![allow(clippy::manual_async_fn)]

#[tracing::instrument(level = "trace", skip(conn))]
pub fn list_users<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    offset: Option<i64>,
    limit: Option<i64>,
) -> impl std::future::Future<Output = Result<(Vec<model::user::User>, i64, i64), anyhow::Error>>
       + Send
       + 'a {
    async move {
        let mut conn = conn.acquire().await?;
        let limit = limit.unwrap_or(20);
        let offset = offset.unwrap_or(0);
        let rows = sqlx::query_as!(
            model::user::User,
            r#"
            SELECT
                users.user_id AS user_id
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OAuthProvider {
    Github(i64, String),
    Facebook(i64, String),
    Twitter(i64, String),
}

#[tracing::instrument(level = "trace", skip(conn))]
pub fn create_user<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    provider: OAuthProvider,
) -> impl std::future::Future<Output = Result<model::user::User, anyhow::Error>> + Send + 'a {
    async move {
        use sqlx::Connection;
        let mut conn = conn.acquire().await?;
        let mut tx = conn.begin().await?;
        let (identifier, username, identity_provider_name) = match provider {
            OAuthProvider::Github(github_id, login) => (github_id.to_string(), login, "github"),
            OAuthProvider::Facebook(facebook_id, name) => {
                (facebook_id.to_string(), name, "facebook")
            }
            OAuthProvider::Twitter(twitter_id, screen_name) => {
                (twitter_id.to_string(), screen_name, "twitter")
            }
        };
        let identities = sqlx::query_as!(
            model::user::UserIdentity,
            r#"
            SELECT
                user_id,
                identifier,
                username,
                identity_provider_name
            FROM user_auths_with_identity_providers
            WHERE 
                user_auths_with_identity_providers.identity_provider_name = ?1
            AND user_auths_with_identity_providers.identifier = ?2
            "#,
            identity_provider_name,
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
            INSERT INTO users DEFAULT VALUES
            RETURNING user_id
            "#
        )
        .fetch_one(&mut *tx)
        .await?;
        let identity_provider_id = sqlx::query!(
            r#"
            SELECT identity_provider_id
            FROM identity_providers
            WHERE identity_provider_name = ?1
            "#,
            identity_provider_name
        )
        .fetch_one(&mut *tx)
        .await?
        .identity_provider_id;
        // アカウント情報を登録
        sqlx::query!(
            r#"
            INSERT INTO user_auths (user_id, identity_provider_id, identifier, username)
            VALUES (?1, ?2, ?3, ?4);
            "#,
            user.user_id,
            identity_provider_id,
            identifier,
            username
        )
        .execute(&mut *tx)
        .await?;
        let user = get_user(&mut tx, user.user_id).await?.unwrap();
        tx.commit().await?;
        Ok(user)
    }
}

// 多重ログイン
#[tracing::instrument(level = "trace", skip(conn))]
pub fn update_user<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    user_id: i64,
    provider: Option<OAuthProvider>,
) -> impl std::future::Future<Output = Result<model::user::User, anyhow::Error>> + Send + 'a {
    async move {
        use sqlx::Connection;
        let mut conn = conn.acquire().await?;
        let mut tx = conn.begin().await?;
        if let Some(provider) = provider {
            let (identifier, username, identity_provider_name) = match provider {
                OAuthProvider::Github(github_id, login) => (github_id.to_string(), login, "github"),
                OAuthProvider::Facebook(facebook_id, name) => {
                    (facebook_id.to_string(), name, "facebook")
                }
                OAuthProvider::Twitter(twitter_id, screen_name) => {
                    (twitter_id.to_string(), screen_name, "twitter")
                }
            };
            let identities = sqlx::query_as!(
                model::user::UserIdentity,
                r#"
                SELECT
                    user_id,
                    identifier,
                    username,
                    identity_provider_name
                FROM user_auths_with_identity_providers
                WHERE 
                    user_auths_with_identity_providers.identity_provider_name = ?1
                AND user_auths_with_identity_providers.identifier = ?2
                "#,
                identity_provider_name,
                identifier
            )
            .fetch_all(&mut *tx)
            .await?;
            if let Some(ident) = identities.first() {
                let user = get_user(&mut *tx, ident.user_id).await?.unwrap();
                // 既に登録済みの場合はそのまま返す
                return Ok(user);
            }
            let identity_provider_id = sqlx::query!(
                r#"
                SELECT identity_provider_id
                FROM identity_providers
                WHERE identity_provider_name = ?1
                "#,
                identity_provider_name
            )
            .fetch_one(&mut *tx)
            .await?
            .identity_provider_id;
            sqlx::query!(
                r#"
                INSERT INTO user_auths (user_id, identity_provider_id, identifier, username)
                VALUES (?1, ?2, ?3, ?4);
                "#,
                user_id,
                identity_provider_id,
                identifier,
                username
            )
            .execute(&mut *tx)
            .await?;
        }
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
                users.user_id AS user_id
            FROM users
            WHERE users.user_id = ?1
            "#,
            id
        )
        .fetch_optional(&mut *conn)
        .await?;
        Ok(row)
    }
}

//#[tracing::instrument(level="trace", skip(conn))]
//pub fn delete_user<'a, 'c>(
//    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
//    id: i64,
//) -> impl std::future::Future<Output = Result<(), anyhow::Error>> + Send + 'a {
//    async move {
//        let mut conn = conn.acquire().await?;
//        sqlx::query!(
//            r#"
//        DELETE
//        FROM users
//        WHERE id = ?1
//        "#,
//            id
//        )
//        .execute(&mut *conn)
//        .await?;
//        Ok(())
//    }
//}

#[tracing::instrument(level = "trace", skip(conn))]
pub fn check_permission<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    user_id: i64,
    req: &'a model::api::Request,
) -> impl std::future::Future<Output = Result<bool, anyhow::Error>> + Send + 'a {
    async move {
        let mut conn = conn.acquire().await?;
        let row = sqlx::query!(
            r#"
            SELECT
                roles.role_name AS role_name
            FROM users
            JOIN roles ON users.role_id = roles.role_id
            WHERE users.user_id = ?1
            "#,
            user_id
        )
        .fetch_one(&mut *conn)
        .await?;
        if row.role_name == "admin" {
            return Ok(true);
        }
        if row.role_name == "default" {
            let flag = match req {
                model::api::Request::GetMe(..) => true,
                model::api::Request::ListUsers(..) => false,
                model::api::Request::ListAccessLogs(..) => false,
                model::api::Request::ListRivers(..) => true,
                model::api::Request::ListRiverWaypoints(..) => true,
                // model::api::Request::CreateRiverWaypoint(..) => false,
            };
            return Ok(flag);
        }
        Ok(false)
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
    user_id: Option<i64>,
    offset: Option<i64>,
    limit: Option<i64>,
) -> impl std::future::Future<Output = Result<(Vec<model::user::AccessLog>, i64, i64), anyhow::Error>>
       + Send
       + 'a {
    async move {
        let mut conn = conn.acquire().await?;
        let limit = limit.unwrap_or(20);
        let offset = offset.unwrap_or(0);
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
