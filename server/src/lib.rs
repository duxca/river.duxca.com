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
    pub base_url: String,
    pub local_client_id: oauth2::ClientId,
    pub local_client_secret: oauth2::ClientSecret,
    pub local_base_url: String,
    pub local_dist_path: String,
    pub gcs_bucket_name: String,
}

#[derive(Clone)]
struct CanonicalBaseUrl(String);

async fn redirect_to_canonical_host(
    axum::extract::State(CanonicalBaseUrl(base_url)): axum::extract::State<CanonicalBaseUrl>,
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    use axum::response::IntoResponse;

    let Ok(base_uri) = base_url.parse::<axum::http::Uri>() else {
        return next.run(req).await;
    };
    let Some(canonical_authority) = base_uri.authority().map(|authority| authority.as_str()) else {
        return next.run(req).await;
    };
    let request_authority = req
        .headers()
        .get(axum::http::header::HOST)
        .and_then(|host| host.to_str().ok());

    if request_authority == Some(canonical_authority) {
        return next.run(req).await;
    }

    let Some(scheme) = base_uri.scheme_str() else {
        return next.run(req).await;
    };
    let path_and_query = req
        .uri()
        .path_and_query()
        .map(|path_and_query| path_and_query.as_str())
        .unwrap_or("/");
    let redirect_to = format!("{scheme}://{canonical_authority}{path_and_query}");

    (
        axum::http::StatusCode::PERMANENT_REDIRECT,
        [(axum::http::header::LOCATION, redirect_to)],
    )
        .into_response()
}

pub async fn create_app(
    config: Config,
    pool: sqlx::sqlite::SqlitePool,
    session_store: tower_sessions_sqlx_store::SqliteStore,
    gcs: google_cloud_storage::client::Storage,
    gcs_control: google_cloud_storage::client::StorageControl,
) -> Result<axum::Router, anyhow::Error> {
    let leptos_options = leptos::config::get_configuration(None)
        .map(|conf| conf.leptos_options)
        .unwrap_or_else(|_| {
            leptos::config::LeptosOptions::builder()
                .output_name("leptos-browser")
                .site_root(config.local_dist_path.clone())
                .site_pkg_dir("pkg")
                .build()
        });
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
            github_client_id: config.local_client_id.clone(),
            github_client_secret: config.local_client_secret.clone(),
            base_url: config.local_base_url.clone(),
        }
    } else {
        web::login::BackendSettings {
            facebook_client_id: config.facebook_client_id.clone(),
            facebook_client_secret: config.facebook_client_secret.clone(),
            github_client_id: config.github_client_id.clone(),
            github_client_secret: config.github_client_secret.clone(),
            base_url: config.base_url.clone(),
        }
    };

    let backend = web::login::Backend::new(pool.clone(), backend_settings);
    let app_pkg_dir =
        std::path::PathBuf::from(&*leptos_options.site_root).join(&*leptos_options.site_pkg_dir);
    let mut app = axum::Router::new()
        .route("/", axum::routing::get(crate::web::home::home))
        .route("/api", axum::routing::post(crate::web::api::api))
        .route(
            "/api/{*fn_name}",
            axum::routing::post(crate::web::server_fn::server_fn),
        )
        .route("/app", axum::routing::get(crate::web::app::app_shell))
        .route("/app/", axum::routing::get(crate::web::app::app_shell))
        .nest_service("/app/pkg", tower_http::services::ServeDir::new(app_pkg_dir))
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
        .fallback(crate::web::home::home)
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
            crate::web::State::new(config.clone(), pool, gcs, gcs_control, leptos_options)?
        });
    if cfg!(not(feature = "local")) {
        app = app.layer(axum::middleware::from_fn_with_state(
            CanonicalBaseUrl(config.base_url.clone()),
            redirect_to_canonical_host,
        ));
    }

    Ok(app)
}
