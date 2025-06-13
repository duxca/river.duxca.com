use server::Config;

mod web;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), anyhow::Error> {
    shadow_rs::shadow!(build);
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
    let config = envy::from_env::<Config>()?;
    log::debug!("config: {:#?}", config);

    let (app, session_store) = server::create_app_with_session_store(config.clone()).await?;

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
