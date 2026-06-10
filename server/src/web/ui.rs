use leptos::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct AuthProviders {
    pub github: Option<model::user::UserAuth>,
    pub facebook: Option<model::user::UserAuth>,
}

impl AuthProviders {
    pub fn from_auths(auths: &[model::user::UserAuth]) -> Self {
        Self {
            github: auths.iter().find(|a| a.identity_type == 0).cloned(),
            facebook: auths.iter().find(|a| a.identity_type == 1).cloned(),
        }
    }
}

const PAGE_STYLE: &str = r#"
body {
    margin: 0;
    font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    background: #f7f7f5;
    color: #242424;
}
main {
    max-width: 720px;
    margin: 0 auto;
    padding: 48px 20px;
}
h1 {
    margin: 0 0 12px;
    font-size: 28px;
    line-height: 1.2;
}
p {
    line-height: 1.7;
}
dl {
    display: grid;
    grid-template-columns: max-content 1fr;
    gap: 8px 16px;
    margin: 20px 0;
}
dt {
    font-weight: 700;
}
dd {
    margin: 0;
}
form {
    display: inline-block;
    margin: 8px 8px 0 0;
}
section {
    border-top: 1px solid #d8d8d2;
    margin-top: 28px;
    padding-top: 24px;
}
h2 {
    font-size: 18px;
    margin: 0 0 12px;
}
.provider {
    align-items: center;
    border-top: 1px solid #e1e1dc;
    display: flex;
    gap: 16px;
    justify-content: space-between;
    padding: 14px 0;
}
.provider:first-of-type {
    border-top: 0;
}
.provider form {
    margin: 0;
}
.provider-name {
    font-weight: 700;
}
.provider-id {
    color: #5d5d58;
    font-size: 14px;
    margin-top: 4px;
    overflow-wrap: anywhere;
}
button, a.button {
    appearance: none;
    border: 1px solid #222;
    border-radius: 6px;
    background: #222;
    color: #fff;
    cursor: pointer;
    display: inline-block;
    font: inherit;
    padding: 10px 14px;
    text-decoration: none;
    white-space: nowrap;
}
a.button.secondary, button.secondary {
    background: transparent;
    color: #222;
}
code {
    background: #ecece8;
    border-radius: 4px;
    padding: 2px 5px;
}
"#;

#[component]
pub fn HomePage(user: Option<model::user::User>, providers: AuthProviders) -> impl IntoView {
    view! {
        <html lang="ja">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <title>"river.duxca.com"</title>
                <style>{PAGE_STYLE}</style>
            </head>
            <body>
                <main>
                    <h1>"river.duxca.com"</h1>
                    <p>"川下り地図アプリのサーバは動いています。現在は Yew UI を切り離して、ログイン確認用の簡易ページを表示しています。"</p>
                    <HomeContent user=user providers=providers/>
                    <p><a href="/version">"version"</a></p>
                </main>
            </body>
        </html>
    }
}

#[component]
pub fn LoginPage(user: Option<model::user::User>, providers: AuthProviders) -> impl IntoView {
    let title = if user.is_some() {
        "アカウント連携"
    } else {
        "ログイン"
    };

    view! {
        <html lang="ja">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <title>{title}</title>
                <style>{PAGE_STYLE}</style>
            </head>
            <body>
                <main>
                    <LoginContent user=user providers=providers/>
                </main>
            </body>
        </html>
    }
}

#[component]
fn HomeContent(user: Option<model::user::User>, providers: AuthProviders) -> impl IntoView {
    match user {
        Some(user) => view! {
            <dl>
                <dt>"Status"</dt>
                <dd>"ログイン済み"</dd>
                <dt>"User ID"</dt>
                <dd><code>{user.user_id}</code></dd>
                <dt>"Nickname"</dt>
                <dd>{user.nickname}</dd>
                <dt>"Role"</dt>
                <dd><code>{user.role}</code></dd>
            </dl>
            <ConnectedAccounts providers=providers/>
            <form method="post" action="/logout">
                <button class="secondary" type="submit">"Logout"</button>
            </form>
        }
        .into_any(),
        None => view! {
            <dl>
                <dt>"Status"</dt>
                <dd>"未ログイン"</dd>
            </dl>
            <form method="post" action="/login/github">
                <button type="submit">"Login with GitHub"</button>
            </form>
            <form method="post" action="/login/facebook">
                <button type="submit">"Login with Facebook"</button>
            </form>
            <a class="button secondary" href="/login">"Provider status"</a>
        }
        .into_any(),
    }
}

#[component]
fn LoginContent(user: Option<model::user::User>, providers: AuthProviders) -> impl IntoView {
    match user {
        Some(user) => view! {
            <h1>"アカウント連携"</h1>
            <p>"ログイン済みのアカウントに、別のログイン方法を追加できます。"</p>
            <dl>
                <dt>"User ID"</dt>
                <dd><code>{user.user_id}</code></dd>
                <dt>"Nickname"</dt>
                <dd>{user.nickname}</dd>
            </dl>
            <ConnectedAccounts providers=providers/>
            <p>
                <a href="/" class="button secondary">"Back"</a>
            </p>
        }
        .into_any(),
        None => view! {
            <h1>"ログイン"</h1>
            <p>"ログイン方法を選んでください。ログイン後に別のログイン方法を同じアカウントへ連携できます。"</p>
            <section>
                <ProviderRow
                    name="GitHub"
                    identifier=None
                    action="/login/github"
                    button_label="Login with GitHub"
                />
                <ProviderRow
                    name="Facebook"
                    identifier=None
                    action="/login/facebook"
                    button_label="Login with Facebook"
                />
            </section>
        }
        .into_any(),
    }
}

#[component]
fn ConnectedAccounts(providers: AuthProviders) -> impl IntoView {
    view! {
        <section>
            <h2>"Connected accounts"</h2>
            <ProviderRow
                name="GitHub"
                identifier=providers.github.map(|auth| auth.identifier)
                action="/login/github"
                button_label="Connect GitHub"
            />
            <ProviderRow
                name="Facebook"
                identifier=providers.facebook.map(|auth| auth.identifier)
                action="/login/facebook"
                button_label="Connect Facebook"
            />
            <a class="button secondary" href="/login">"Manage connections"</a>
        </section>
    }
}

#[component]
fn ProviderRow(
    name: &'static str,
    identifier: Option<String>,
    action: &'static str,
    button_label: &'static str,
) -> impl IntoView {
    let connected = identifier.is_some();
    let status = identifier
        .map(|identifier| format!("connected: {identifier}"))
        .unwrap_or_else(|| "not connected".to_owned());

    view! {
        <div class="provider">
            <div>
                <div class="provider-name">{name}</div>
                <div class="provider-id">{status}</div>
            </div>
            {(!connected).then(|| view! {
                <form method="post" action=action>
                    <button type="submit">{button_label}</button>
                </form>
            })}
        </div>
    }
}
