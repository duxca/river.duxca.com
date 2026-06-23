use anyhow::{Context, Result};
use fantoccini::{Client, ClientBuilder, Locator};
use serde_json::{Map, json};

fn env_url(name: &str, default: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| default.to_string())
}

async fn new_client() -> Result<Client> {
    let webdriver_url = env_url("WEBDRIVER_URL", "http://127.0.0.1:9515");
    let mut caps = Map::new();
    caps.insert(
        "goog:chromeOptions".to_string(),
        json!({
            "args": [
                "--headless=new",
                "--no-sandbox",
                "--disable-dev-shm-usage",
                "--window-size=1280,900"
            ]
        }),
    );
    ClientBuilder::native()
        .capabilities(caps)
        .connect(&webdriver_url)
        .await
        .with_context(|| format!("failed to connect to WebDriver at {webdriver_url}"))
}

async fn body_text(client: &Client) -> Result<String> {
    client
        .find(Locator::Css("body"))
        .await?
        .text()
        .await
        .context("body text is missing")
}

async fn wait_for_body_text(client: &Client, text: &str) -> Result<String> {
    let mut body = String::new();
    for _ in 0..40 {
        body = body_text(client).await.unwrap_or_default();
        if body.contains(text) {
            return Ok(body);
        }
        tokio::time::sleep(std::time::Duration::from_millis(250)).await;
    }
    anyhow::bail!(
        "body did not contain {text:?}; url={} body={body}",
        client.current_url().await?
    );
}

async fn login_with_fake_github(client: &Client, server_url: &str) -> Result<()> {
    client.goto(&format!("{server_url}/login")).await?;
    client
        .find(Locator::Css("form[action='/login/github'] button"))
        .await?
        .click()
        .await?;
    wait_for_body_text(client, "ログイン済み").await?;
    Ok(())
}

#[tokio::test]
async fn server_home_shows_public_login_choice() -> Result<()> {
    let server_url = env_url("SERVER_URL", "http://127.0.0.1:18080");
    let client = new_client().await?;

    client.goto(&server_url).await?;
    let body = body_text(&client).await?;

    assert!(body.contains("river.duxca.com"));
    assert!(body.contains("Facebook"));
    assert!(!body.contains("Login with GitHub"));
    assert!(!body.contains("Provider status"));
    assert!(!body.contains("version"));
    assert!(client.find(Locator::Css("a[href='/app']")).await.is_err());
    assert!(client.find(Locator::Css("a[href='/login']")).await.is_err());
    assert!(
        client
            .find(Locator::Css("a[href='/version']"))
            .await
            .is_err()
    );

    client.close().await?;
    Ok(())
}

#[tokio::test]
async fn login_page_shows_provider_buttons() -> Result<()> {
    let server_url = env_url("SERVER_URL", "http://127.0.0.1:18080");
    let client = new_client().await?;

    client.goto(&format!("{server_url}/login")).await?;
    let body = body_text(&client).await?;

    assert!(body.contains("GitHub"));
    assert!(body.contains("Facebook"));

    client.close().await?;
    Ok(())
}

#[tokio::test]
async fn unknown_path_returns_not_found() -> Result<()> {
    let server_url = env_url("SERVER_URL", "http://127.0.0.1:18080");
    let client = new_client().await?;

    client.goto(&format!("{server_url}/unknown-path")).await?;
    let status = client
        .execute(
            "return window.performance.getEntriesByType('navigation')[0].responseStatus",
            vec![],
        )
        .await?;
    let body = body_text(&client).await?;

    assert_eq!(status.as_u64(), Some(404));
    assert!(body.contains("404 not found"));

    client.close().await?;
    Ok(())
}

#[tokio::test]
async fn fake_facebook_login_creates_regular_user() -> Result<()> {
    let server_url = env_url("SERVER_URL", "http://127.0.0.1:18080");
    let client = new_client().await?;

    client.goto(&server_url).await?;
    client
        .find(Locator::Css("form[action='/login/facebook'] button"))
        .await?
        .click()
        .await?;

    let mut body = String::new();
    for _ in 0..40 {
        body = body_text(&client).await.unwrap_or_default();
        if body.contains("ログイン済み") {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_millis(250)).await;
    }
    assert!(
        body.contains("ログイン済み"),
        "url={} body={body}",
        client.current_url().await?
    );
    assert!(body.contains("fake-facebook-user"));
    assert!(body.contains("Role"));
    assert!(body.contains("1"));
    assert!(!body.contains("管理画面"));
    assert!(
        client
            .find(Locator::Css("form[action='/login/github'] button"))
            .await
            .is_err()
    );

    client.close().await?;
    Ok(())
}

#[tokio::test]
async fn fake_github_login_creates_session() -> Result<()> {
    let server_url = env_url("SERVER_URL", "http://127.0.0.1:18080");
    let client = new_client().await?;

    login_with_fake_github(&client, &server_url).await?;
    let body = body_text(&client).await?;
    assert!(body.contains("fake-github-user"));
    assert!(body.contains("Role"));
    assert!(body.contains("0"));
    assert!(body.contains("管理画面"));
    assert!(!body.contains("Manage connections"));
    assert!(
        client
            .find(Locator::Css("form[action='/login/github'] button"))
            .await
            .is_err()
    );

    client.goto(&format!("{server_url}/login")).await?;
    let body = body_text(&client).await?;
    assert_eq!(
        client.current_url().await?.as_str(),
        &format!("{server_url}/")
    );
    assert!(body.contains("ログイン済み"));
    assert!(body.contains("fake-github-user"));
    assert!(body.contains("Connect Facebook"));
    assert!(!body.contains("Manage connections"));

    client.goto(&format!("{server_url}/app")).await?;
    assert_eq!(client.title().await?, "river.duxca.com Leptos map");

    client.close().await?;
    Ok(())
}

#[tokio::test]
async fn map_layers_and_mobile_layout_work_after_login() -> Result<()> {
    let server_url = env_url("SERVER_URL", "http://127.0.0.1:18080");
    let client = new_client().await?;

    login_with_fake_github(&client, &server_url).await?;
    client.goto(&format!("{server_url}/app")).await?;

    let options = client
        .execute(
            "return Array.from(document.querySelectorAll('select option')).map((option) => option.textContent)",
            vec![],
        )
        .await?;
    let options = options
        .as_array()
        .context("layer selector options should be an array")?;
    for label in [
        "地理院タイル",
        "赤色立体図",
        "OpenStreetMap",
        "陰影起伏図",
        "白地図",
        "航空写真",
    ] {
        assert!(
            options.iter().any(|option| option.as_str() == Some(label)),
            "missing layer option {label:?}: {options:?}"
        );
    }

    client
        .find(Locator::Css("select"))
        .await?
        .select_by_value("red-relief")
        .await?;
    let red_relief = client
        .execute(
            "return {
                value: document.querySelector('select')?.value,
                tiles: Array.from(document.querySelectorAll('img.leaflet-tile')).map((tile) => tile.src),
            }",
            vec![],
        )
        .await?;
    assert_eq!(red_relief["value"].as_str(), Some("red-relief"));
    let tiles = red_relief["tiles"]
        .as_array()
        .context("red relief tiles should be an array")?;
    assert!(
        tiles.iter().any(|tile| tile
            .as_str()
            .is_some_and(|src| src.contains("/xyz/sekishoku/"))),
        "red relief tile was not requested: {tiles:?}"
    );

    client.set_window_rect(0, 0, 390, 844).await?;
    let layout = client
        .execute(
            "const map = document.querySelector('.leaflet-container')?.getBoundingClientRect();
             const app = document.querySelector('.app-shell')?.getBoundingClientRect();
             return {
                mapWidth: map?.width,
                appWidth: app?.width,
                viewportWidth: window.innerWidth,
                layerLabel: document.querySelector('.map-controls label:first-child')?.getBoundingClientRect().width,
             }",
            vec![],
        )
        .await?;
    let viewport_width = layout["viewportWidth"]
        .as_f64()
        .context("viewportWidth should be numeric")?;
    let map_width = layout["mapWidth"]
        .as_f64()
        .context("mapWidth should be numeric")?;
    let app_width = layout["appWidth"]
        .as_f64()
        .context("appWidth should be numeric")?;
    let layer_label_width = layout["layerLabel"]
        .as_f64()
        .context("layerLabel should be numeric")?;

    assert!(
        map_width <= viewport_width,
        "map overflows viewport: {layout}"
    );
    assert!(
        app_width <= viewport_width,
        "app overflows viewport: {layout}"
    );
    assert!(
        layer_label_width >= viewport_width - 40.0,
        "layer selector should have enough mobile width: {layout}"
    );

    client.close().await?;
    Ok(())
}

#[tokio::test]
async fn leptos_app_loads_browser_bundle() -> Result<()> {
    let frontend_url = env_url("FRONTEND_URL", "http://127.0.0.1:18080/app");
    let client = new_client().await?;

    client.goto(&frontend_url).await?;

    assert_eq!(client.title().await?, "river.duxca.com Leptos map");

    let source = client.source().await?;
    assert!(source.contains("/app/pkg/frontend"));

    client.close().await?;
    Ok(())
}
