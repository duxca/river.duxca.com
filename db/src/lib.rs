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

    // ここで remote db に対して migrate する
    sqlx::migrate!().run(&pool).await?;
    Ok(pool)
}
