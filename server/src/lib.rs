pub mod web;

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

impl Config {
    pub fn test_config() -> Self {
        Self {
            host_addr: "127.0.0.1:0".to_string(),
            database_url: ":memory:".to_string(),
            github_client_id: oauth2::ClientId::new("test_github_client_id".to_string()),
            github_client_secret: oauth2::ClientSecret::new("test_github_client_secret".to_string()),
            facebook_client_id: oauth2::ClientId::new("test_facebook_client_id".to_string()),
            facebook_client_secret: oauth2::ClientSecret::new("test_facebook_client_secret".to_string()),
            twitter_client_id: oauth2::ClientId::new("test_twitter_client_id".to_string()),
            twitter_client_secret: oauth2::ClientSecret::new("test_twitter_client_secret".to_string()),
            base_url: "http://localhost:3000".to_string(),
            local_client_id: oauth2::ClientId::new(
                std::env::var("LOCAL_CLIENT_ID")
                    .unwrap_or_else(|_| "test_local_client_id".to_string())
            ),
            local_client_secret: oauth2::ClientSecret::new(
                std::env::var("LOCAL_CLIENT_SECRET")
                    .unwrap_or_else(|_| "test_local_client_secret".to_string())
            ),
            local_base_url: "http://localhost:3000".to_string(),
            local_dist_path: "dist".to_string(),
            gcs_bucket_name: "test_bucket".to_string(),
            gcp_credentials_file: "test_credentials.json".to_string(),
        }
    }
}

pub async fn create_app(config: Config) -> Result<axum::Router, anyhow::Error> {
    let (app, _) = create_app_with_session_store(config).await?;
    Ok(app)
}

pub async fn create_app_with_session_store(config: Config) -> Result<(axum::Router, tower_sessions_sqlx_store::SqliteStore), anyhow::Error> {
    let gcs = if config.gcp_credentials_file.is_empty() || config.gcp_credentials_file == "test_credentials.json" {
        // For testing - use anonymous client
        let conf = google_cloud_storage::client::ClientConfig::default()
            .anonymous();
        google_cloud_storage::client::Client::new(conf)
    } else {
        // For production - use credentials file
        let cred = google_cloud_auth::credentials::CredentialsFile::new_from_file(
            config.gcp_credentials_file.clone(),
        ).await?;
        let conf = google_cloud_storage::client::ClientConfig::default()
            .with_credentials(cred)
            .await?;
        google_cloud_storage::client::Client::new(conf)
    };

    let pool = db::connect(&config.database_url).await?;
    let session_store = tower_sessions_sqlx_store::SqliteStore::new(pool.clone());
    session_store.migrate().await?;

    let mut session_layer = tower_sessions::SessionManagerLayer::new(session_store.clone())
        .with_same_site(tower_sessions::cookie::SameSite::Lax)
        .with_expiry(tower_sessions::Expiry::OnInactivity(
            std::time::Duration::from_secs(7 * 24 * 60 * 60).try_into()?,
        ));
    
    if cfg!(not(feature = "local")) {
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
        .route("/api", axum::routing::post(web::api::api))
        .layer(tower_http::cors::CorsLayer::very_permissive())
        .route("/image", axum::routing::post(web::image::upload_image))
        .route("/image/{image_id}", axum::routing::get(web::image::get_image))
        .route("/image/{image_id}", axum::routing::delete(web::image::delete_image))
        .route("/admin", axum::routing::get(web::admin::admin))
        .route("/admin/apply", axum::routing::post(web::admin::admin_apply))
        .route("/admin/delete_waypoints", axum::routing::post(web::admin::admin_delete_waypoints))
        .route("/login", axum::routing::get(web::login::login))
        .route("/login/github", axum::routing::post(web::login::github::login))
        .route("/oauth/callback/github", axum::routing::get(web::login::github::callback))
        .route("/login/twitter", axum::routing::post(web::login::twitter::login))
        .route("/oauth/callback/twitter", axum::routing::get(web::login::twitter::callback))
        .route("/login/facebook", axum::routing::post(web::login::facebook::login))
        .route("/oauth/callback/facebook", axum::routing::get(web::login::facebook::callback))
        .route("/logout", axum::routing::post(web::login::logout))
        .route("/logout", axum::routing::get(web::login::logout))
        .route("/version", axum::routing::get(|| async { 
            env!("CARGO_PKG_VERSION")
        }))
        .fallback_service({
            use axum::handler::HandlerWithoutStateExt;
            tower_http::services::ServeDir::new(if cfg!(feature = "local") {
                &config.local_dist_path
            } else {
                "dist"
            })
            .not_found_service(web::handler_404.into_service())
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
        .with_state(web::State::new(config, pool, gcs)?);

    Ok((app, session_store))
}