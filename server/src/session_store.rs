//! SQLite session store for tower-sessions.
//!
//! tower-sessions-sqlx-store has no release built on sqlx 0.9, so the
//! SQLite store is implemented in-house. The schema and the record
//! encoding (MessagePack via rmp-serde, table `tower_sessions`) match
//! tower-sessions-sqlx-store 0.15 so existing session rows stay readable.

use async_trait::async_trait;
use sqlx::{SqliteConnection, sqlite::SqlitePool};
use time::OffsetDateTime;
use tower_sessions_core::{
    SessionStore,
    session::{Id, Record},
    session_store::{self, ExpiredDeletion},
};

#[derive(Debug, thiserror::Error)]
pub enum SqliteStoreError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Encode(#[from] rmp_serde::encode::Error),
    #[error(transparent)]
    Decode(#[from] rmp_serde::decode::Error),
}

impl From<SqliteStoreError> for session_store::Error {
    fn from(err: SqliteStoreError) -> Self {
        match err {
            SqliteStoreError::Sqlx(inner) => session_store::Error::Backend(inner.to_string()),
            SqliteStoreError::Decode(inner) => session_store::Error::Decode(inner.to_string()),
            SqliteStoreError::Encode(inner) => session_store::Error::Encode(inner.to_string()),
        }
    }
}

/// A SQLite session store.
#[derive(Clone, Debug)]
pub struct SqliteStore {
    pool: SqlitePool,
    table_name: String,
}

impl SqliteStore {
    /// Create a new SQLite store with the provided connection pool.
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool,
            table_name: "tower_sessions".into(),
        }
    }

    /// Set the session table name with the provided name.
    pub fn with_table_name(mut self, table_name: impl AsRef<str>) -> Result<Self, String> {
        let table_name = table_name.as_ref();
        if !is_valid_table_name(table_name) {
            return Err(format!(
                "Invalid table name '{table_name}'. Table names must be alphanumeric and may contain \
                 hyphens or underscores."
            ));
        }

        table_name.clone_into(&mut self.table_name);
        Ok(self)
    }

    /// Migrate the session schema.
    pub async fn migrate(&self) -> sqlx::Result<()> {
        let query = format!(
            r#"
            create table if not exists {}
            (
                id text primary key not null,
                data blob not null,
                expiry_date integer not null
            )
            "#,
            self.table_name
        );
        sqlx::query(sqlx::AssertSqlSafe(query))
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn try_create_with_conn(
        &self,
        conn: &mut SqliteConnection,
        record: &Record,
    ) -> session_store::Result<bool> {
        let query = format!(
            r#"
            insert or abort into {table_name}
              (id, data, expiry_date) values (?, ?, ?)
            "#,
            table_name = self.table_name
        );
        let res = sqlx::query(sqlx::AssertSqlSafe(query))
            .bind(record.id.to_string())
            .bind(rmp_serde::to_vec(record).map_err(SqliteStoreError::Encode)?)
            .bind(record.expiry_date)
            .execute(conn)
            .await;

        match res {
            Ok(_) => Ok(true),
            Err(sqlx::Error::Database(e)) if e.is_unique_violation() => Ok(false),
            Err(e) => Err(SqliteStoreError::Sqlx(e).into()),
        }
    }
}

#[async_trait]
impl ExpiredDeletion for SqliteStore {
    async fn delete_expired(&self) -> session_store::Result<()> {
        let query = format!(
            r#"
            delete from {table_name}
            where datetime(expiry_date) < datetime('now')
            "#,
            table_name = self.table_name
        );
        sqlx::query(sqlx::AssertSqlSafe(query))
            .execute(&self.pool)
            .await
            .map_err(SqliteStoreError::Sqlx)?;
        Ok(())
    }
}

#[async_trait]
impl SessionStore for SqliteStore {
    async fn create(&self, record: &mut Record) -> session_store::Result<()> {
        let mut tx = self.pool.begin().await.map_err(SqliteStoreError::Sqlx)?;

        while !self.try_create_with_conn(&mut tx, record).await? {
            record.id = Id::default();
        }

        tx.commit().await.map_err(SqliteStoreError::Sqlx)?;

        Ok(())
    }

    async fn save(&self, record: &Record) -> session_store::Result<()> {
        let mut conn = self.pool.acquire().await.map_err(SqliteStoreError::Sqlx)?;
        let query = format!(
            r#"
            insert into {table_name}
              (id, data, expiry_date) values (?, ?, ?)
            on conflict(id) do update set
              data = excluded.data,
              expiry_date = excluded.expiry_date
            "#,
            table_name = self.table_name
        );
        sqlx::query(sqlx::AssertSqlSafe(query))
            .bind(record.id.to_string())
            .bind(rmp_serde::to_vec(record).map_err(SqliteStoreError::Encode)?)
            .bind(record.expiry_date)
            .execute(&mut *conn)
            .await
            .map_err(SqliteStoreError::Sqlx)?;
        Ok(())
    }

    async fn load(&self, session_id: &Id) -> session_store::Result<Option<Record>> {
        let query = format!(
            r#"
            select data from {}
            where id = ? and expiry_date > ?
            "#,
            self.table_name
        );
        let data: Option<(Vec<u8>,)> = sqlx::query_as(sqlx::AssertSqlSafe(query))
            .bind(session_id.to_string())
            .bind(OffsetDateTime::now_utc())
            .fetch_optional(&self.pool)
            .await
            .map_err(SqliteStoreError::Sqlx)?;

        if let Some((data,)) = data {
            Ok(Some(
                rmp_serde::from_slice(&data).map_err(SqliteStoreError::Decode)?,
            ))
        } else {
            Ok(None)
        }
    }

    async fn delete(&self, session_id: &Id) -> session_store::Result<()> {
        let query = format!(
            r#"
            delete from {} where id = ?
            "#,
            self.table_name
        );
        sqlx::query(sqlx::AssertSqlSafe(query))
            .bind(session_id.to_string())
            .execute(&self.pool)
            .await
            .map_err(SqliteStoreError::Sqlx)?;
        Ok(())
    }
}

fn is_valid_table_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}
