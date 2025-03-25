mod web;

#[derive(serde::Deserialize, Debug)]
struct Config {
    host_addr: String,
    database_url: String,
    github_client_id: oauth2::ClientId,
    github_client_secret: oauth2::ClientSecret,
    facebook_client_id: oauth2::ClientId,
    facebook_client_secret: oauth2::ClientSecret,
    twitter_client_id: oauth2::ClientId,
    twitter_client_secret: oauth2::ClientSecret,
    base_url: String,
    local_client_id: oauth2::ClientId,
    local_client_secret: oauth2::ClientSecret,
    local_base_url: String,
    local_dist_path: String,
}

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

    let pool = db::connect(&config.database_url).await?;
    let session_store = tower_sessions_sqlx_store::SqliteStore::new(pool.clone());
    // セッションテーブルの作成
    session_store.migrate().await?;

    //let mut rdr = csv::ReaderBuilder::new()
    //    .delimiter(b',')
    //    .quote(b'"')
    //    .has_headers(false)
    //    .trim(csv::Trim::All)
    //    .from_path("./field_spot.csv")?;
    //for result in rdr.deserialize::<model::field::FieldSpotCsv>() {
    //    let spot = result?;
    //    let mut conn = pool.acquire().await?;
    //    println!("{:?}", spot);
    //    crate::db::field::upsert_field_spot(
    //        &mut *conn,
    //        spot.field_name,
    //        spot.spot_name,
    //        spot.longitude,
    //        spot.latitude,
    //    )
    //    .await?;
    //}

    // cookie のセッションの設定
    let mut session_layer = tower_sessions::SessionManagerLayer::new(session_store.clone())
        // NOTE:oauth でリダイレクトするときに　SameSite::Strict だとエラーになる
        .with_same_site(tower_sessions::cookie::SameSite::Lax)
        .with_expiry(tower_sessions::Expiry::OnInactivity(
            // 24h
            std::time::Duration::from_secs(7 * 24 * 60 * 60).try_into()?,
        ));
    if cfg!(not(feature = "local")) {
        // 本番環境で有効にする
        session_layer = session_layer.with_secure(true).with_http_only(true);
    }
    let backend_settings = if cfg!(feature = "local") {
        crate::web::login::BackendSettings {
            facebook_client_id: config.facebook_client_id.clone(),
            facebook_client_secret: config.facebook_client_secret.clone(),
            twitter_client_id: config.twitter_client_id.clone(),
            twitter_client_secret: config.twitter_client_secret.clone(),
            github_client_id: config.local_client_id.clone(),
            github_client_secret: config.local_client_secret.clone(),
            base_url: config.local_base_url.clone(),
        }
    } else {
        crate::web::login::BackendSettings {
            facebook_client_id: config.facebook_client_id.clone(),
            facebook_client_secret: config.facebook_client_secret.clone(),
            twitter_client_id: config.twitter_client_id.clone(),
            twitter_client_secret: config.twitter_client_secret.clone(),
            github_client_id: config.github_client_id.clone(),
            github_client_secret: config.github_client_secret.clone(),
            base_url: config.base_url.clone(),
        }
    };
    let backend = crate::web::login::Backend::new(pool.clone(), backend_settings);
    let app = axum::Router::new()
        .route(
            "/version",
            axum::routing::get(|| async { build::CLAP_LONG_VERSION }),
        )
        .route("/api", axum::routing::post(crate::web::api::api))
        .route(
            "/login/github",
            axum::routing::post(crate::web::login::github::login),
        )
        .route(
            "/oauth/callback/github",
            axum::routing::get(crate::web::login::github::callback),
        )
        .route(
            "/login/twitter",
            axum::routing::post(crate::web::login::twitter::login),
        )
        .route(
            "/oauth/callback/twitter",
            axum::routing::get(crate::web::login::twitter::callback),
        )
        .route(
            "/login/facebook",
            axum::routing::post(crate::web::login::facebook::login),
        )
        .route(
            "/oauth/callback/facebook",
            axum::routing::get(crate::web::login::facebook::callback),
        )
        .route("/logout", axum::routing::post(crate::web::login::logout))
        .layer(axum_login::AuthManagerLayerBuilder::new(backend, session_layer).build())
        .layer(tower_http::cors::CorsLayer::very_permissive())
        .fallback_service(if cfg!(feature = "local") {
            tower_http::services::ServeDir::new(config.local_dist_path)
        } else {
            tower_http::services::ServeDir::new("dist")
        })
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(tower_http::compression::CompressionLayer::new())
        .with_state({
            // 一般のリクエストで DB にアクセスするための State
            crate::web::State::from_pool(pool)?
        });

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
