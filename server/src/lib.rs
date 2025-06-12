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
            local_client_id: oauth2::ClientId::new("test_local_client_id".to_string()),
            local_client_secret: oauth2::ClientSecret::new("test_local_client_secret".to_string()),
            local_base_url: "http://localhost:3000".to_string(),
            local_dist_path: "dist".to_string(),
            gcs_bucket_name: "test_bucket".to_string(),
            gcp_credentials_file: "test_credentials.json".to_string(),
        }
    }
}

pub async fn create_test_app() -> Result<axum::Router, anyhow::Error> {
    env_logger::builder().is_test(true).try_init().ok();
    
    let config = Config::test_config();
    let pool = db::connect(&config.database_url).await?;
    
    let session_store = tower_sessions_sqlx_store::SqliteStore::new(pool.clone());
    session_store.migrate().await?;

    let session_layer = tower_sessions::SessionManagerLayer::new(session_store.clone())
        .with_same_site(tower_sessions::cookie::SameSite::Lax)
        .with_expiry(tower_sessions::Expiry::OnInactivity(
            std::time::Duration::from_secs(24 * 60 * 60).try_into()?,
        ));

    let backend_settings = web::login::BackendSettings {
        facebook_client_id: config.facebook_client_id.clone(),
        facebook_client_secret: config.facebook_client_secret.clone(),
        twitter_client_id: config.twitter_client_id.clone(),
        twitter_client_secret: config.twitter_client_secret.clone(),
        github_client_id: config.github_client_id.clone(),
        github_client_secret: config.github_client_secret.clone(),
        base_url: config.base_url.clone(),
    };
    
    let backend = web::login::Backend::new(pool.clone(), backend_settings);

    let gcs = {
        let conf = google_cloud_storage::client::ClientConfig::default()
            .anonymous();
        google_cloud_storage::client::Client::new(conf)
    };

    let app = axum::Router::new()
        .route("/api", axum::routing::post(web::api::api))
        .route("/admin", axum::routing::get(web::admin::admin))
        .route("/login", axum::routing::get(web::login::login))
        .route("/login/github", axum::routing::post(web::login::github::login))
        .route("/oauth/callback/github", axum::routing::get(web::login::github::callback))
        .route("/login/twitter", axum::routing::post(web::login::twitter::login))
        .route("/oauth/callback/twitter", axum::routing::get(web::login::twitter::callback))
        .route("/login/facebook", axum::routing::post(web::login::facebook::login))
        .route("/oauth/callback/facebook", axum::routing::get(web::login::facebook::callback))
        .route("/logout", axum::routing::post(web::login::logout))
        .route("/logout", axum::routing::get(web::login::logout))
        .fallback_service({
            use axum::handler::HandlerWithoutStateExt;
            tower_http::services::ServeDir::new("dist")
                .not_found_service(web::handler_404.into_service())
        })
        .layer(axum_login::AuthManagerLayerBuilder::new(backend, session_layer).build())
        .with_state(web::State::new(config, pool, gcs)?);

    Ok(app)
}