#![recursion_limit = "256"]

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), anyhow::Error> {
    let _ = any_spawner::Executor::init_tokio();
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    dotenvy::dotenv().ok();
    // env_logger::init();
    tracing_subscriber::fmt()
        .with_timer(tracing_subscriber::fmt::time::LocalTime::rfc_3339())
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_file(true)
        .with_line_number(true)
        .with_thread_names(true)
        .with_thread_ids(true)
        .init();
    let config = envy::from_env::<server::Config>()?;

    let pool = db::connect(&config.database_url).await?;
    let session_store = tower_sessions_sqlx_store::SqliteStore::new(pool.clone());
    // セッションテーブルの作成
    session_store.migrate().await?;

    let app = server::create_app(config.clone(), pool, session_store.clone()).await?;
    let app = app.into_make_service_with_connect_info::<std::net::SocketAddr>();

    let listener = tokio::net::TcpListener::bind(config.host_addr).await?;
    // セッションの定期削除タスク
    // NOTE: tokio::task::spawn を rt=current_thread で使うと single thread で動く
    let deletion_task = tokio::task::spawn({
        let oneday = std::time::Duration::from_secs(60 * 60 * 24);
        tower_sessions_core::ExpiredDeletion::continuously_delete_expired(session_store, oneday)
    });
    let deletion_task_abort_handle = deletion_task.abort_handle();
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
    let server = axum::serve(listener, app).with_graceful_shutdown(async move {
        let ctrl_c = async {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to install Ctrl+C handler");
        };
        let terminate = async {
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                .expect("failed to install signal handler")
                .recv()
                .await;
        };
        tokio::select! {
            _ = ctrl_c => {},
            _ = terminate => {},
        }
        // 止まるのは期限切れセッションの定期削除だけ。セッションの保存はリクエストごとに
        // SessionManagerLayer が SqliteStore へ書くので影響しない。
        // abort しないと下の deletion_task.await が返らず、SIGTERM で終了しない
        deletion_task_abort_handle.abort();
        let _ = shutdown_tx.send(());
    });
    // litestream は server の終了を待ってから最終同期するので、Cloud Run の SIGKILL (10 秒) に
    // 間に合うよう処理中リクエストを待つのは 3 秒までにする
    tokio::select! {
        result = server.into_future() => result?,
        _ = async {
            let _ = shutdown_rx.await;
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        } => tracing::warn!("graceful shutdown timed out, dropping in-flight requests"),
    }

    match deletion_task.await {
        Ok(Ok(())) => {
            // nop
        }
        Ok(Err(e)) => {
            // session の削除タスクが異常終了
            Err(e)?;
        }
        Err(e) if e.is_cancelled() => {
            // nop
        }
        Err(e) => {
            // task が panic
            Err(e)?;
        }
    }
    Ok(())
}
