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
    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
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
            // これすると sqlite の中のセッションが永続化しないので開発時のみ使う
            tokio::select! {
                _ = ctrl_c => {
                    if cfg!(feature = "local") {
                        deletion_task_abort_handle.abort()
                    }
                },
                _ = terminate => {
                    if cfg!(feature = "local") {
                        deletion_task_abort_handle.abort()
                    }
                },
            }
        })
        .await?;

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
