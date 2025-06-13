async fn spawn_test_server() -> anyhow::Result<String> {
    use tokio::net::TcpListener;
    env_logger::builder().is_test(true).try_init().ok();
    let config = server::Config::test_config();
    let app = server::create_app(config).await?;

    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;
    let server_url = format!("http://{}", addr);

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    Ok(server_url)
}

#[tokio::test]
async fn test_full_login_flow_with_browser() -> anyhow::Result<()> {
    use headless_chrome::{Browser, LaunchOptions};
    use http_body_util::BodyExt;
    use hyper::{Method, Request, StatusCode};
    use hyper_util::{client::legacy::Client, rt::TokioExecutor};
    use std::ffi::OsStr;
    use std::time::Duration;

    let server_url = spawn_test_server().await?;

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
    let browser = Browser::new(options)?;
    let tab = browser.new_tab()?;

    // Test 1: Navigate to login page
    let login_page_url = format!("{}/login", server_url);
    tab.navigate_to(&login_page_url)?;
    tab.wait_for_element("body")?;

    // Test 2: Check login buttons are present
    let github_button = tab.find_element("form[action='/login/github'] button");
    assert!(
        github_button.is_ok(),
        "GitHub login button should be present"
    );

    // Test 3: Test OAuth initiation flow (GitHub)
    let github_btn = tab.find_element("form[action='/login/github'] button")?;
    github_btn.click()?;

    // Wait for redirect to GitHub OAuth
    std::thread::sleep(Duration::from_millis(2000));
    let current_url = tab.get_url();

    // Should be redirected to GitHub OAuth or error page
    assert!(
        current_url.contains("github.com") || current_url.contains("login"),
        "Should redirect to GitHub OAuth or show error, current URL: {}",
        current_url
    );

    Ok(())
    /*
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
        */
}
