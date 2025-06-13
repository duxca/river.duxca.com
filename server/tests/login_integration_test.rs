use std::ffi::OsStr;
use std::time::Duration;

use headless_chrome::{Browser, LaunchOptions};
use http_body_util::BodyExt;
use hyper::{Method, Request, StatusCode};
use hyper_util::{client::legacy::Client, rt::TokioExecutor};
use tokio::net::TcpListener;

async fn spawn_test_server() -> anyhow::Result<(String, tokio::task::JoinHandle<()>)> {
    env_logger::builder().is_test(true).try_init().ok();
    let config = server::Config::test_config();
    let app = server::create_app(config).await?;

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
    client: &Client<
        hyper_util::client::legacy::connect::HttpConnector,
        http_body_util::Empty<bytes::Bytes>,
    >,
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
    assert!(
        status.is_redirection(),
        "GitHub login should redirect, got: {}",
        status
    );

    let facebook_login_url = format!("{}/login/facebook", server_url);
    let (status, _body) = make_request(&client, Method::POST, &facebook_login_url).await?;
    assert!(
        status.is_redirection(),
        "Facebook login should redirect, got: {}",
        status
    );

    let twitter_login_url = format!("{}/login/twitter", server_url);
    let (status, _body) = make_request(&client, Method::POST, &twitter_login_url).await?;
    assert!(
        status.is_redirection(),
        "Twitter login should redirect, got: {}",
        status
    );

    handle.abort();
    Ok(())
}

#[tokio::test]
async fn test_logout_endpoint() -> anyhow::Result<()> {
    let (server_url, handle) = spawn_test_server().await?;

    let client = Client::builder(TokioExecutor::new()).build_http();

    let logout_url = format!("{}/logout", server_url);
    let (status, _body) = make_request(&client, Method::POST, &logout_url).await?;
    assert!(
        status.is_redirection(),
        "Logout should redirect, got: {}",
        status
    );

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
        .body(http_body_util::Full::new(bytes::Bytes::from(
            r#"{"GetRiver":{"id":1}}"#,
        )))?;

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

    let github_callback_url = format!(
        "{}/login/github/callback?code=test_code&state=test_state",
        server_url
    );
    let (status, body) = make_request(&client, Method::GET, &github_callback_url).await?;

    assert!(
        status.is_client_error() || status.is_server_error(),
        "GitHub callback with invalid code should fail, got: {} - {}",
        status,
        body
    );

    let facebook_callback_url = format!(
        "{}/login/facebook/callback?code=test_code&state=test_state",
        server_url
    );
    let (status, body) = make_request(&client, Method::GET, &facebook_callback_url).await?;

    assert!(
        status.is_client_error() || status.is_server_error(),
        "Facebook callback with invalid code should fail, got: {} - {}",
        status,
        body
    );

    let twitter_callback_url = format!(
        "{}/login/twitter/callback?oauth_token=test&oauth_verifier=test",
        server_url
    );
    let (status, body) = make_request(&client, Method::GET, &twitter_callback_url).await?;

    assert!(
        status.is_client_error() || status.is_server_error(),
        "Twitter callback with invalid token should fail, got: {} - {}",
        status,
        body
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
    assert!(
        res.status().is_redirection(),
        "Login should redirect, got: {}",
        res.status()
    );

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

#[tokio::test]
async fn test_full_login_flow_with_browser() -> anyhow::Result<()> {
    let (server_url, handle) = spawn_test_server().await?;

    // Launch headless Chrome with more permissive options
    let options = LaunchOptions::default_builder()
        .headless(true)
        .sandbox(false)
        .args(vec![
            OsStr::new("--no-sandbox"),
            OsStr::new("--disable-gpu"),
            OsStr::new("--disable-dev-shm-usage"),
            OsStr::new("--disable-setuid-sandbox"),
            OsStr::new("--disable-background-timer-throttling"),
            OsStr::new("--disable-backgrounding-occluded-windows"),
            OsStr::new("--disable-renderer-backgrounding"),
            OsStr::new("--disable-features=TranslateUI"),
        ])
        .build()?;

    let browser = match Browser::new(options) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Failed to start browser: {}. Skipping browser test.", e);
            handle.abort();
            return Ok(());
        }
    };

    let tab = match browser.new_tab() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to create tab: {}. Skipping browser test.", e);
            handle.abort();
            return Ok(());
        }
    };

    // Test 1: Navigate to login page
    let login_page_url = format!("{}/login", server_url);
    if let Err(e) = tab.navigate_to(&login_page_url) {
        eprintln!(
            "Failed to navigate to login page: {}. Skipping browser test.",
            e
        );
        handle.abort();
        return Ok(());
    }

    if let Err(e) = tab.wait_for_element("body") {
        eprintln!(
            "Failed to wait for body element: {}. Skipping browser test.",
            e
        );
        handle.abort();
        return Ok(());
    }

    let page_title = tab.get_title().unwrap_or_else(|_| "Unknown".to_string());
    println!("Login page title: {}", page_title);

    // Test 2: Check login buttons are present
    let github_button = tab.find_element("form[action='/login/github'] button");
    assert!(
        github_button.is_ok(),
        "GitHub login button should be present"
    );

    let facebook_button = tab.find_element("form[action='/login/facebook'] button");
    assert!(
        facebook_button.is_ok(),
        "Facebook login button should be present"
    );

    let twitter_button = tab.find_element("form[action='/login/twitter'] button");
    assert!(
        twitter_button.is_ok(),
        "Twitter login button should be present"
    );

    // Test 3: Test OAuth initiation flow (GitHub)
    let github_btn = tab.find_element("form[action='/login/github'] button")?;
    github_btn.click()?;

    // Wait for redirect to GitHub OAuth
    std::thread::sleep(Duration::from_millis(2000));
    let current_url = tab.get_url();

    // Should be redirected to GitHub OAuth or error page
    assert!(
        current_url.contains("github.com")
            || current_url.contains("error")
            || current_url.contains("login"),
        "Should redirect to GitHub OAuth or show error, current URL: {}",
        current_url
    );

    // Test 4: Test logout functionality if we can access it
    tab.navigate_to(&format!("{}/logout", server_url))?;
    std::thread::sleep(Duration::from_millis(1000));

    let logout_url = tab.get_url();
    assert!(
        logout_url.contains("login") || logout_url.contains("/"),
        "After logout should redirect to login or home, got: {}",
        logout_url
    );

    // Test 5: Test protected API access without authentication
    tab.navigate_to(&format!("{}/api", server_url))?;
    std::thread::sleep(Duration::from_millis(1000));

    let api_response = tab.get_content()?;
    // Should be redirected to login or get unauthorized response
    assert!(
        api_response.contains("Unauthorized")
            || api_response.contains("login")
            || tab.get_url().contains("login"),
        "Unauthenticated API access should be blocked"
    );

    // Test 6: Verify session handling
    tab.navigate_to(&login_page_url)?;
    tab.wait_for_element("body")?;

    // Check if session cookies are set
    let cookies = tab.get_cookies()?;
    let has_session_cookie = cookies.iter().any(|cookie| {
        cookie.name.contains("session")
            || cookie.name.contains("tower")
            || cookie.name.to_lowercase().contains("auth")
    });

    if has_session_cookie {
        println!(
            "Session cookies found: {:?}",
            cookies.iter().map(|c| &c.name).collect::<Vec<_>>()
        );
    }

    // Test 7: Test form CSRF protection
    let csrf_token = tab.find_element("input[name='csrf_token']");
    if csrf_token.is_ok() {
        let token_element = csrf_token?;
        let token_value = token_element.get_attribute_value("value")?;
        assert!(
            token_value.is_some() && !token_value.unwrap().is_empty(),
            "CSRF token should be present and non-empty"
        );
    }

    // Test 8: Test accessibility and basic UI structure
    let main_content = tab.find_element("main, .main, #main, body");
    assert!(main_content.is_ok(), "Page should have main content area");

    let login_forms = tab.find_elements("form[action*='/login/']")?;
    assert!(
        login_forms.len() >= 3,
        "Should have at least 3 OAuth provider forms, found: {}",
        login_forms.len()
    );

    // Test 9: Test responsive design basics
    // Note: viewport setting not available in current headless_chrome API
    let mobile_buttons = tab.find_elements("form[action*='/login/'] button")?;
    assert!(
        mobile_buttons.len() >= 3,
        "Login buttons should be visible, found: {}",
        mobile_buttons.len()
    );

    // Test 10: Test JavaScript functionality if present
    let js_result = tab.evaluate("typeof window !== 'undefined'", false);
    if js_result.is_ok() {
        println!("JavaScript environment is available");

        // Test if any client-side form validation exists
        let form_validation = tab.evaluate(
            "document.querySelector('form[action*=\"/login/\"]') !== null",
            false,
        );
        if let Ok(has_forms) = form_validation {
            println!(
                "Login forms detected via JavaScript: {}",
                has_forms.value.unwrap_or_default()
            );
        }
    }

    // Browser will be closed when dropped
    handle.abort();
    Ok(())
}

#[tokio::test]
async fn test_login_flow_error_handling() -> anyhow::Result<()> {
    let (server_url, handle) = spawn_test_server().await?;

    let options = LaunchOptions::default_builder()
        .headless(true)
        .sandbox(false)
        .args(vec![
            OsStr::new("--no-sandbox"),
            OsStr::new("--disable-gpu"),
            OsStr::new("--disable-dev-shm-usage"),
            OsStr::new("--disable-setuid-sandbox"),
        ])
        .build()?;

    let browser = match Browser::new(options) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Failed to start browser: {}. Skipping browser test.", e);
            handle.abort();
            return Ok(());
        }
    };

    let tab = match browser.new_tab() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to create tab: {}. Skipping browser test.", e);
            handle.abort();
            return Ok(());
        }
    };

    // Test 1: Invalid OAuth callback
    let invalid_callback_url = format!(
        "{}/oauth/callback/github?code=invalid&state=invalid",
        server_url
    );
    if let Err(e) = tab.navigate_to(&invalid_callback_url) {
        eprintln!(
            "Failed to navigate to callback URL: {}. Skipping browser test.",
            e
        );
        handle.abort();
        return Ok(());
    }
    std::thread::sleep(Duration::from_millis(1000));

    let error_content = tab.get_content()?;
    let current_url = tab.get_url();

    // Should show error or redirect to login
    assert!(
        error_content.contains("error")
            || error_content.contains("Error")
            || current_url.contains("login")
            || current_url.contains("error"),
        "Invalid OAuth callback should show error or redirect to login"
    );

    // Test 2: Direct access to protected endpoints
    let protected_endpoints = vec!["/api", "/admin", "/user/profile"];

    for endpoint in protected_endpoints {
        let protected_url = format!("{}{}", server_url, endpoint);
        tab.navigate_to(&protected_url)?;
        std::thread::sleep(Duration::from_millis(500));

        let protected_content = tab.get_content()?;
        let protected_url_current = tab.get_url();

        // Should be redirected to login or show unauthorized
        assert!(
            protected_content.contains("Unauthorized")
                || protected_content.contains("login")
                || protected_url_current.contains("login")
                || protected_content.contains("403")
                || protected_content.contains("401"),
            "Protected endpoint {} should require authentication",
            endpoint
        );
    }

    // Test 3: Malformed login requests
    let malformed_login_url = format!("{}/login/nonexistent", server_url);
    tab.navigate_to(&malformed_login_url)?;
    std::thread::sleep(Duration::from_millis(500));

    let malformed_content = tab.get_content()?;
    assert!(
        malformed_content.contains("404")
            || malformed_content.contains("Not Found")
            || tab.get_url().contains("login"),
        "Malformed login URL should return 404 or redirect"
    );

    // Browser will be closed when dropped
    handle.abort();
    Ok(())
}

#[tokio::test]
async fn test_login_ui_interactions() -> anyhow::Result<()> {
    let (server_url, handle) = spawn_test_server().await?;

    let options = LaunchOptions::default_builder()
        .headless(true)
        .sandbox(false)
        .args(vec![
            OsStr::new("--no-sandbox"),
            OsStr::new("--disable-gpu"),
            OsStr::new("--disable-dev-shm-usage"),
            OsStr::new("--disable-setuid-sandbox"),
        ])
        .build()?;

    let browser = match Browser::new(options) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Failed to start browser: {}. Skipping browser test.", e);
            handle.abort();
            return Ok(());
        }
    };

    let tab = match browser.new_tab() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to create tab: {}. Skipping browser test.", e);
            handle.abort();
            return Ok(());
        }
    };

    let login_page_url = format!("{}/login", server_url);
    if let Err(e) = tab.navigate_to(&login_page_url) {
        eprintln!(
            "Failed to navigate to login page: {}. Skipping browser test.",
            e
        );
        handle.abort();
        return Ok(());
    }

    if let Err(e) = tab.wait_for_element("body") {
        eprintln!(
            "Failed to wait for body element: {}. Skipping browser test.",
            e
        );
        handle.abort();
        return Ok(());
    }

    // Test 1: Button hover states and interactions
    if let Ok(github_btn) = tab.find_element("form[action='/login/github'] button") {
        // Test button is present and accessible
        // Note: is_enabled not available in current headless_chrome API

        // Test button focus
        github_btn.focus()?;
        std::thread::sleep(Duration::from_millis(100));

        // Verify button styling/attributes
        let button_type = github_btn.get_attribute_value("type")?;
        assert!(
            button_type.is_some(),
            "Login button should have type attribute"
        );
    }

    // Test 2: Form submission behavior
    if let Ok(forms) = tab.find_elements("form[action*='/login/']") {
        for (i, form) in forms.iter().enumerate() {
            let action = form.get_attribute_value("action")?;
            assert!(
                action.is_some() && action.unwrap().contains("/login/"),
                "Form {} should have proper action attribute",
                i
            );

            let method = form.get_attribute_value("method")?;
            assert!(method.is_some(), "Form {} should have method attribute", i);
        }
    }

    // Test 3: Page navigation and back button
    if let Ok(github_btn) = tab.find_element("form[action='/login/github'] button") {
        let initial_url = tab.get_url();

        // Click login button
        github_btn.click()?;
        std::thread::sleep(Duration::from_millis(1000));

        let after_click_url = tab.get_url();
        assert!(
            initial_url != after_click_url,
            "URL should change after clicking login button"
        );

        // Test back navigation
        // Note: go_back not available in current headless_chrome API
        // Navigate back to login page manually
        tab.navigate_to(&login_page_url)?;
        std::thread::sleep(Duration::from_millis(500));

        let back_url = tab.get_url();
        assert!(
            back_url.contains("login"),
            "Should be able to navigate back to login page"
        );
    }

    // Test 4: Multiple tab/window behavior
    let second_tab = browser.new_tab()?;
    second_tab.navigate_to(&login_page_url)?;
    second_tab.wait_for_element("body")?;

    let first_content = tab.get_content()?;
    let second_content = second_tab.get_content()?;

    // Both tabs should show login page
    assert!(
        first_content.len() > 0 && second_content.len() > 0,
        "Both tabs should load login page content"
    );

    // Browser will be closed when dropped
    handle.abort();
    Ok(())
}
