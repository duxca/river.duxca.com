use yew::prelude::*;
use yew::suspense::use_future;

#[derive(Debug, PartialEq, Clone)]
enum PageState {
    LoggedOut,
    LoggedIn(model::user::User),
}

#[hook]
fn use_user() -> yew::suspense::SuspensionResult<Option<model::user::User>> {
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
