shadow_rs::shadow!(build);

mod web;

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Config {
    pub host_addr: String,
    pub database_url: String,
    pub github_client_id: oauth2::ClientId,
    pub github_client_secret: oauth2::ClientSecret,
    pub facebook_client_id: oauth2::ClientId,
    pub facebook_client_secret: oauth2::ClientSecret,
    pub twitter_client_id: oauth2::ClientId,
    pub twitter_client_secret: oauth2::ClientSecret,
    pub base_url: String,
    pub local_client_id: oauth2::ClientId,
    pub local_client_secret: oauth2::ClientSecret,
    pub local_base_url: String,
    pub local_dist_path: String,
    pub gcs_bucket_name: String,
    pub gcp_credentials_file: String,
}

pub async fn create_app(
    config: Config,
    pool: sqlx::sqlite::SqlitePool,
    session_store: tower_sessions_sqlx_store::SqliteStore,
    gcs: google_cloud_storage::client::Storage,
    gcs_control: google_cloud_storage::client::StorageControl,
) -> Result<axum::Router, anyhow::Error> {
    let mut session_layer = tower_sessions::SessionManagerLayer::new(session_store.clone())
        .with_same_site(tower_sessions::cookie::SameSite::Lax)
        .with_expiry(tower_sessions::Expiry::OnInactivity(
            std::time::Duration::from_secs(7 * 24 * 60 * 60).try_into()?,
        ));
    if cfg!(not(feature = "local")) {
        // 本番環境で有効にする
        session_layer = session_layer.with_secure(true).with_http_only(true);
    }

    let backend_settings = if cfg!(feature = "local") {
        web::login::BackendSettings {
            facebook_client_id: config.facebook_client_id.clone(),
            facebook_client_secret: config.facebook_client_secret.clone(),
            twitter_client_id: config.twitter_client_id.clone(),
            twitter_client_secret: config.twitter_client_secret.clone(),
            github_client_id: config.local_client_id.clone(),
            github_client_secret: config.local_client_secret.clone(),
            base_url: config.local_base_url.clone(),
        }
    } else {
        web::login::BackendSettings {
            facebook_client_id: config.facebook_client_id.clone(),
            facebook_client_secret: config.facebook_client_secret.clone(),
            twitter_client_id: config.twitter_client_id.clone(),
            twitter_client_secret: config.twitter_client_secret.clone(),
            github_client_id: config.github_client_id.clone(),
            github_client_secret: config.github_client_secret.clone(),
            base_url: config.base_url.clone(),
        }
    };

    let backend = web::login::Backend::new(pool.clone(), backend_settings);
    let app = axum::Router::new()
        .route("/api", axum::routing::post(crate::web::api::api))
        .layer(tower_http::cors::CorsLayer::very_permissive())
        .route(
            "/image",
            axum::routing::post(crate::web::image::upload_image),
        )
        .route(
            "/image/{image_id}",
            axum::routing::get(crate::web::image::get_image),
        )
        .route(
            "/image/{image_id}",
            axum::routing::delete(crate::web::image::delete_image),
        )
        .route("/admin", axum::routing::get(crate::web::admin::admin))
        .route(
            "/admin/apply",
            axum::routing::post(crate::web::admin::admin_apply),
        )
        .route(
            "/admin/delete_waypoints",
            axum::routing::post(crate::web::admin::admin_delete_waypoints),
        )
        .route("/login", axum::routing::get(crate::web::login::login))
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
        .route("/logout", axum::routing::get(crate::web::login::logout))
        .route(
            "/version",
            axum::routing::get(|| async { build::CLAP_LONG_VERSION }),
        )
        .fallback_service({
            use axum::handler::HandlerWithoutStateExt;
            tower_http::services::ServeDir::new(if cfg!(feature = "local") {
                &config.local_dist_path
            } else {
                "dist"
            })
            .not_found_service(crate::web::handler_404.into_service())
        })
        .layer(axum_login::AuthManagerLayerBuilder::new(backend, session_layer).build())
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(tower_http::compression::CompressionLayer::new())
        .layer(tower_default_headers::DefaultHeadersLayer::new({
            let mut default_headers = axum::http::header::HeaderMap::new();
            default_headers.insert(
                axum::http::header::CACHE_CONTROL,
                axum::http::header::HeaderValue::from_static("no-store"),
            );
            default_headers
        }))
        .with_state({
            // 一般のリクエストで DB にアクセスするための State
            crate::web::State::new(config.clone(), pool, gcs, gcs_control)?
        });

    Ok(app)
}
