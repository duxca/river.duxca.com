pub mod river_tracks;
pub mod river_waypoints;
pub mod rivers;
pub mod user;

pub async fn connect(database_url: &str) -> Result<sqlx::sqlite::SqlitePool, anyhow::Error> {
    // NOTE: litestream 用の option
    let pool = sqlx::sqlite::SqlitePool::connect_with({
        use std::str::FromStr;
        sqlx::sqlite::SqliteConnectOptions::from_str(database_url)?
            .foreign_keys(true)
            // https://litestream.io/tips/#disable-autocheckpoints-for-high-write-load-servers
            .pragma("wal_autocheckpoint", "0")
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
            // https://litestream.io/tips/#busy-timeout
            .busy_timeout(std::time::Duration::from_secs(5))
            // https://litestream.io/tips/#synchronous-pragma
            .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
    })
    .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn connect_runs_migrations() -> Result<(), anyhow::Error> {
        let db_path = std::env::temp_dir().join(format!(
            "river-db-migration-test-{}-{}.sqlite3",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_nanos()
        ));
        let database_url = format!("sqlite://{}?mode=rwc", db_path.display());

        let pool = super::connect(&database_url).await?;
        let files_table: Option<(i64,)> =
            sqlx::query_as("SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = 'files'")
                .fetch_optional(&pool)
                .await?;
        assert!(files_table.is_none());

        pool.close().await;
        let _ = std::fs::remove_file(&db_path);
        let _ = std::fs::remove_file(db_path.with_extension("sqlite3-shm"));
        let _ = std::fs::remove_file(db_path.with_extension("sqlite3-wal"));
        Ok(())
    }
}
