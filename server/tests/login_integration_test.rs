use std::time::Duration;

use hyper::{Method, Request, StatusCode};
use hyper_util::{client::legacy::Client, rt::TokioExecutor};
use tokio::net::TcpListener;
use http_body_util::BodyExt;

async fn spawn_test_server() -> anyhow::Result<(String, tokio::task::JoinHandle<()>)> {
    let app = server::create_test_app().await?;
    
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;
    let server_url = format!("http://{}", addr);
    
    let handle = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    Ok((server_url, handle))
}

async fn make_request(
    client: &Client<hyper_util::client::legacy::connect::HttpConnector, http_body_util::Empty<bytes::Bytes>>,
    method: Method,
    url: &str,
) -> anyhow::Result<(StatusCode, String)> {
    let req = Request::builder()
        .method(method)
        .uri(url)
        .header("user-agent", "test-client")
        .body(http_body_util::Empty::<bytes::Bytes>::new())?;
    
    let res = client.request(req).await?;
    let status = res.status();
    let body_bytes = res.into_body().collect().await?.to_bytes();
    let body = std::string::String::from_utf8(body_bytes.to_vec())?;
    
    Ok((status, body))
}

#[tokio::test]
async fn test_login_endpoints_exist() -> anyhow::Result<()> {
    let (server_url, handle) = spawn_test_server().await?;
    
    let client = Client::builder(TokioExecutor::new()).build_http();
    
    let github_login_url = format!("{}/login/github", server_url);
    let (status, _body) = make_request(&client, Method::POST, &github_login_url).await?;
    assert!(status.is_redirection(), "GitHub login should redirect, got: {}", status);
    
    let facebook_login_url = format!("{}/login/facebook", server_url);
    let (status, _body) = make_request(&client, Method::POST, &facebook_login_url).await?;
    assert!(status.is_redirection(), "Facebook login should redirect, got: {}", status);
    
    let twitter_login_url = format!("{}/login/twitter", server_url);
    let (status, _body) = make_request(&client, Method::POST, &twitter_login_url).await?;
    assert!(status.is_redirection(), "Twitter login should redirect, got: {}", status);
    
    handle.abort();
    Ok(())
}

#[tokio::test]
async fn test_logout_endpoint() -> anyhow::Result<()> {
    let (server_url, handle) = spawn_test_server().await?;
    
    let client = Client::builder(TokioExecutor::new()).build_http();
    
    let logout_url = format!("{}/logout", server_url);
    let (status, _body) = make_request(&client, Method::POST, &logout_url).await?;
    assert!(status.is_redirection(), "Logout should redirect, got: {}", status);
    
    handle.abort();
    Ok(())
}

#[tokio::test]
async fn test_unauthorized_api_access() -> anyhow::Result<()> {
    let (server_url, handle) = spawn_test_server().await?;
    
    let client = Client::builder(TokioExecutor::new()).build_http();
    
    let api_url = format!("{}/api", server_url);
    let req = Request::builder()
        .method(Method::POST)
        .uri(&api_url)
        .header("content-type", "application/json")
        .body(http_body_util::Full::new(bytes::Bytes::from(r#"{"GetRiver":{"id":1}}"#)))?;
    
    let res = client.request(req).await?;
    let status = res.status();
    
    assert!(
        status == StatusCode::UNAUTHORIZED || status == StatusCode::FOUND,
        "Unauthenticated API access should return 401 or redirect, got: {}", 
        status
    );
    
    handle.abort();
    Ok(())
}

#[tokio::test]
async fn test_session_cookie_handling() -> anyhow::Result<()> {
    let (server_url, handle) = spawn_test_server().await?;
    
    let client = Client::builder(TokioExecutor::new()).build_http();
    
    let login_page_url = format!("{}/login", server_url);
    let req = Request::builder()
        .method(Method::GET)
        .uri(&login_page_url)
        .body(http_body_util::Empty::<bytes::Bytes>::new())?;
    
    let res = client.request(req).await?;
    let status = res.status();
    
    assert_eq!(status, StatusCode::OK, "Login page should be accessible");
    
    let headers = res.headers();
    let set_cookie_headers: Vec<_> = headers.get_all("set-cookie").iter().collect();
    
    if !set_cookie_headers.is_empty() {
        println!("Session cookies set: {:?}", set_cookie_headers);
    }
    
    handle.abort();
    Ok(())
}

#[tokio::test]
async fn test_oauth_callback_endpoints() -> anyhow::Result<()> {
    let (server_url, handle) = spawn_test_server().await?;
    
    let client = Client::builder(TokioExecutor::new()).build_http();
    
    let github_callback_url = format!("{}/login/github/callback?code=test_code&state=test_state", server_url);
    let (status, body) = make_request(&client, Method::GET, &github_callback_url).await?;
    
    assert!(
        status.is_client_error() || status.is_server_error(),
        "GitHub callback with invalid code should fail, got: {} - {}", 
        status, body
    );
    
    let facebook_callback_url = format!("{}/login/facebook/callback?code=test_code&state=test_state", server_url);
    let (status, body) = make_request(&client, Method::GET, &facebook_callback_url).await?;
    
    assert!(
        status.is_client_error() || status.is_server_error(),
        "Facebook callback with invalid code should fail, got: {} - {}", 
        status, body
    );
    
    let twitter_callback_url = format!("{}/login/twitter/callback?oauth_token=test&oauth_verifier=test", server_url);
    let (status, body) = make_request(&client, Method::GET, &twitter_callback_url).await?;
    
    assert!(
        status.is_client_error() || status.is_server_error(),
        "Twitter callback with invalid token should fail, got: {} - {}", 
        status, body
    );
    
    handle.abort();
    Ok(())
}

#[tokio::test]
async fn test_login_flow_redirects() -> anyhow::Result<()> {
    let (server_url, handle) = spawn_test_server().await?;
    
    let client = Client::builder(TokioExecutor::new()).build_http();
    
    let github_login_url = format!("{}/login/github", server_url);
    let req = Request::builder()
        .method(Method::POST)
        .uri(&github_login_url)
        .body(http_body_util::Empty::<bytes::Bytes>::new())?;
    
    let res = client.request(req).await?;
    assert!(res.status().is_redirection(), "Login should redirect, got: {}", res.status());
    
    let location = res.headers().get("location");
    assert!(location.is_some(), "Login should include redirect location");
    
    let location_str = location.unwrap().to_str()?;
    assert!(
        location_str.contains("github.com"),
        "GitHub login should redirect to GitHub, got: {}", 
        location_str
    );
    
    handle.abort();
    Ok(())
}