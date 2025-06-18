//! Login component handling user authentication.
//!
//! # Usage
//!
//! ```rust
//! use yew::prelude::*;
//!
//! #[function_component(App)]
//! pub fn app() -> Html {
//!     html! {
//!         <Suspense fallback={html!{<div>{"Loading..."}</div>}}>
//!             <Login />
//!         </Suspense>
//!     }
//! }
//! ```
//!
//! The Login component automatically checks authentication status and renders either:
//! - Login form with Twitter/GitHub OAuth buttons when logged out
//! - Home component when logged in

use yew::prelude::*;
use yew::suspense::use_future;

#[derive(Debug, PartialEq, Clone)]
enum PageState {
    LoggedOut,
    LoggedIn(model::user::User),
}

#[hook]
fn use_user() -> yew::suspense::SuspensionResult<Option<model::user::User>> {
    #[cfg(feature = "test-mode")]
    {
        let test_user = model::user::User {
            id: 1,
            name: "Test User".to_string(),
            email: Some("test@example.com".to_string()),
            image_url: None,
            role: model::user::Role::User,
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
        };
        return Ok(Some(test_user));
    }
    
    #[cfg(not(feature = "test-mode"))]
    {
        let s = use_future(|| async {
            let res =
                crate::api::call::<model::api::get_me::Response>(model::api::get_me::Request {}).await;
            match res {
                Ok(res) => Some(res.user),
                Err(_) => None,
            }
        })?;
        Ok((*s).clone())
    }
}

#[function_component(Login)]
#[allow(clippy::redundant_closure)]
pub fn login() -> HtmlResult {
    let page_state = use_state_eq(|| PageState::LoggedOut);
    // 初回のみログインチェック
    let user = use_user()?;
    if let Some(user) = &user {
        page_state.set(PageState::LoggedIn(user.clone()));
    }

    let html = match &*page_state {
        PageState::LoggedOut => {
            html! {
                <>
                <form method="post" action="/login/twitter">
                    <input type="submit" value="twitter Login" />
                </form>
                <form method="post" action="/login/github">
                    <input type="submit" value="github Login" />
                </form>
                </>
            }
        }
        PageState::LoggedIn(user) => {
            html! {
                <crate::components::home::Home
                    user={user.clone()}
                />
            }
        }
    };
    Ok(html)
}
