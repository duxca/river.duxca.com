mod api;
mod auth;
mod db;
mod web;

#[derive(serde::Deserialize, Debug)]
struct Config {
    host_addr: String,
    database_url: String,
    github_client_id: oauth2::ClientId,
    github_client_secret: oauth2::ClientSecret,
    facebook_client_id: oauth2::ClientId,
    facebook_client_secret: oauth2::ClientSecret,
    redirect_url: oauth2::RedirectUrl,
    local_client_id: oauth2::ClientId,
    local_client_secret: oauth2::ClientSecret,
    local_redirect_url: oauth2::RedirectUrl,
    local_dist_path: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), anyhow::Error> {
    shadow_rs::shadow!(build);
    dotenvy::dotenv().ok();
    //env_logger::init();
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
    use std::str::FromStr;
    let opt = sqlx::sqlite::SqliteConnectOptions::from_str(&config.database_url)?
        .foreign_keys(true)
        // https://litestream.io/tips/#disable-autocheckpoints-for-high-write-load-servers
        .pragma("wal_autocheckpoint", "0")
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        // https://litestream.io/tips/#busy-timeout
        .busy_timeout(std::time::Duration::from_secs(5))
        // https://litestream.io/tips/#synchronous-pragma
        .synchronous(sqlx::sqlite::SqliteSynchronous::Normal);
    let pool = sqlx::sqlite::SqlitePool::connect_with(opt).await?;

    // ここで remote db に対して migrate する
    sqlx::migrate!().run(&pool).await?;

    let session_store = tower_sessions_sqlx_store::SqliteStore::new(pool.clone());
    // セッションテーブルの作成
    session_store.migrate().await?;

    // セッションの定期削除タスク
    // tokio::task::spawn を rt=current_thread で使うと single thread で動く
    let deletion_task = tokio::task::spawn({
        use tower_sessions::ExpiredDeletion;
        session_store
            .clone()
            .continuously_delete_expired(tokio::time::Duration::from_secs(60 * 60 * 24))
    });

    // cookie のセッションの設定
    let mut session_layer = tower_sessions::SessionManagerLayer::new(session_store)
        // oauth でリダイレクトするときにStrict だとエラーになる
        //.with_same_site(tower_sessions::cookie::SameSite::Lax)
        .with_same_site(tower_sessions::cookie::SameSite::Strict)
        .with_expiry(tower_sessions::Expiry::OnInactivity(
            std::time::Duration::from_secs(600).try_into()?,
        ));
    if cfg!(not(feature = "local")) {
        // 本番環境で有効にする
        session_layer = session_layer.with_secure(true).with_http_only(true);
    }
    let tokens = if cfg!(feature = "local") {
        crate::auth::BackendSettings {
            github: crate::auth::ClientToken {
                client_id: config.local_client_id.clone(),
                client_secret: config.local_client_secret.clone(),
            },
            facebook: crate::auth::ClientToken {
                client_id: config.facebook_client_id.clone(),
                client_secret: config.facebook_client_secret.clone(),
            },
            redirect_url: config.local_redirect_url,
        }
    } else {
        crate::auth::BackendSettings {
            github: crate::auth::ClientToken {
                client_id: config.github_client_id.clone(),
                client_secret: config.github_client_secret.clone(),
            },
            facebook: crate::auth::ClientToken {
                client_id: config.facebook_client_id.clone(),
                client_secret: config.facebook_client_secret.clone(),
            },
            redirect_url: config.redirect_url,
        }
    };
    let backend = crate::auth::Backend::new(pool.clone(), tokens);
    // 一般のリクエストで DB にアクセスするための State
    let st = crate::web::State::from_pool(pool)?;

    let app = axum::Router::new()
        .route("/api", axum::routing::post(crate::web::api::api))
        .route("/login", axum::routing::post(crate::web::login::login))
        .route("/logout", axum::routing::post(crate::web::login::logout))
        .route(
            "/oauth/callback",
            axum::routing::get(crate::web::login::callback),
        )
        .layer(axum_login::AuthManagerLayerBuilder::new(backend, session_layer).build())
        .layer(
            tower_http::cors::CorsLayer::very_permissive(), // .allow_credentials(true)
                                                            // .allow_methods(tower_http::cors::Any)
                                                            // .allow_origin(tower_http::cors::Any),
        )
        .nest_service(
            "/",
            if cfg!(feature = "local") {
                tower_http::services::ServeDir::new(config.local_dist_path)
            } else {
                tower_http::services::ServeDir::new("dist")
            },
        )
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(tower_http::compression::CompressionLayer::new())
        .with_state(st);

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
    let deletion_task_abort_handle = deletion_task.abort_handle();

    let listener = tokio::net::TcpListener::bind(config.host_addr).await?;

    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
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
