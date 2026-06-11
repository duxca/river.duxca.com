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

#[derive(Clone, Debug, Default)]
pub struct AccountContext {
    pub csrf_token: Option<String>,
    pub delete_preview: Option<model::user::UserDeletePreview>,
}

#[component]
pub fn HomePage(
    user: Option<model::user::User>,
    providers: AuthProviders,
    account: AccountContext,
) -> impl IntoView {
    view! {
        <html lang="ja">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <title>"river.duxca.com"</title>
                <link rel="stylesheet" href="/app/pkg/leptos-browser.css"/>
            </head>
            <body class="static-page">
                <main>
                    <h1>"river.duxca.com"</h1>
                    <p>"川下り地図アプリのサーバは動いています。"</p>
                    <HomeContent user=user providers=providers account=account/>
                    <p><a class="button secondary" href="/version">"version"</a></p>
                </main>
            </body>
        </html>
    }
}

#[component]
pub fn LoginPage(
    user: Option<model::user::User>,
    providers: AuthProviders,
    account: AccountContext,
) -> impl IntoView {
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
                <link rel="stylesheet" href="/app/pkg/leptos-browser.css"/>
            </head>
            <body class="static-page">
                <main>
                    <LoginContent user=user providers=providers account=account/>
                </main>
            </body>
        </html>
    }
}

#[component]
fn HomeContent(
    user: Option<model::user::User>,
    providers: AuthProviders,
    account: AccountContext,
) -> impl IntoView {
    match user {
        Some(user) => {
            let delete_user = user.clone();
            view! {
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
                <LoggedInNavigation role=user.role/>
                <OptionalAccountDeleteSection user=delete_user account=account/>
                <form method="post" action="/logout">
                    <button class="secondary" type="submit">"Logout"</button>
                </form>
            }
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
fn LoginContent(
    user: Option<model::user::User>,
    providers: AuthProviders,
    account: AccountContext,
) -> impl IntoView {
    match user {
        Some(user) => {
            let delete_user = user.clone();
            view! {
                <h1>"アカウント連携"</h1>
                <p>"ログイン済みのアカウントに、別のログイン方法を追加できます。"</p>
                <dl>
                    <dt>"User ID"</dt>
                    <dd><code>{user.user_id}</code></dd>
                    <dt>"Nickname"</dt>
                    <dd>{user.nickname}</dd>
                </dl>
                <ConnectedAccounts providers=providers/>
                <OptionalAccountDeleteSection user=delete_user account=account/>
                <p>
                    <a href="/" class="button secondary">"Back"</a>
                </p>
            }
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
fn LoggedInNavigation(role: i64) -> impl IntoView {
    view! {
        <section>
            <h2>"Navigation"</h2>
            <p>
                <a class="button" href="/app">"地図アプリ"</a>
                {(role == 0).then(|| view! {
                    <a class="button secondary" href="/admin">"管理画面"</a>
                })}
            </p>
        </section>
    }
}

#[component]
fn OptionalAccountDeleteSection(
    user: model::user::User,
    account: AccountContext,
) -> impl IntoView {
    match (account.csrf_token, account.delete_preview) {
        (Some(csrf_token), Some(preview)) => view! {
            <AccountDeleteSection user=user preview=preview csrf_token=csrf_token/>
        }
        .into_any(),
        _ => ().into_any(),
    }
}

#[component]
fn AccountDeleteSection(
    user: model::user::User,
    preview: model::user::UserDeletePreview,
    csrf_token: String,
) -> impl IntoView {
    if user.role == 0 {
        return view! {
            <section class="danger-zone">
                <h2>"アカウント削除"</h2>
                <p>"管理者アカウントはこの画面から削除できません。"</p>
            </section>
        }
        .into_any();
    }

    view! {
        <section class="danger-zone">
            <h2>"アカウント削除"</h2>
            <p>
                "アカウントを削除すると、ログイン情報と関連データはアーカイブされ、通常の画面からは利用できなくなります。"
                "同じ OAuth で再ログインすると、新しいアカウントが作成されます。"
            </p>
            <ul class="delete-preview">
                <li>"登録した河川: " {preview.river_count}</li>
                <li>"削除対象のトラック: " {preview.track_count}</li>
                <li>"削除対象のウェイポイント: " {preview.waypoint_count}</li>
                <li>"連携済みログイン方法: " {preview.auth_count}</li>
            </ul>
            <form method="post" action="/account/delete">
                <input type="hidden" name="csrf_token" value=csrf_token/>
                <label class="delete-confirm-label" for="nickname_confirm">
                    "確認のためニックネームを入力してください"
                </label>
                <input
                    id="nickname_confirm"
                    name="nickname_confirm"
                    type="text"
                    autocomplete="off"
                    required=true
                    placeholder=user.nickname.clone()
                />
                <label class="delete-confirm-checkbox">
                    <input type="checkbox" name="confirm_delete" value="yes" required=true/>
                    "削除は取り消せず、同じ OAuth で再ログインすると新しいアカウントになることを理解しました"
                </label>
                <button class="danger" type="submit">"アカウントを削除"</button>
            </form>
        </section>
    }
    .into_any()
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