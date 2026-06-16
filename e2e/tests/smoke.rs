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

#[tokio::test]
async fn server_home_shows_login_choices() -> Result<()> {
    let server_url = env_url("SERVER_URL", "http://127.0.0.1:18080");
    let client = new_client().await?;

    client.goto(&server_url).await?;
    let body = body_text(&client).await?;

    assert!(body.contains("river.duxca.com"));
    assert!(body.contains("GitHub"));
    assert!(body.contains("Facebook"));

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
async fn fake_github_login_creates_session() -> Result<()> {
    let server_url = env_url("SERVER_URL", "http://127.0.0.1:18080");
    let client = new_client().await?;

    client.goto(&server_url).await?;
    client
        .find(Locator::Css("form[action='/login/github'] button"))
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
    assert!(body.contains("fake-github-user"));

    client.goto(&format!("{server_url}/app")).await?;
    assert_eq!(client.title().await?, "river.duxca.com Leptos map");

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
    assert!(source.contains("/app/pkg/leptos-browser"));

    client.close().await?;
    Ok(())
}
