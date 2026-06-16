use leptos::config::LeptosOptions;
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
    pub delete_preview: Option<model::user::UserDeletePreview>,
}

#[component]
pub fn HomePage(
    user: Option<model::user::User>,
    providers: AuthProviders,
    account: AccountContext,
    options: LeptosOptions,
) -> impl IntoView {
    let css_path = format!("/app{}", options.css_path());
    view! {
        <html lang="ja">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <title>"river.duxca.com"</title>
                <link rel="stylesheet" href=css_path/>
                <HydrationScripts options=options.clone() islands=true root="/app"/>
            </head>
            <body class="static-page">
                <main>
                    <h1>"river.duxca.com"</h1>
                    <p>"川下り地図アプリのサーバは動いています。"</p>
                    <p>
                        <a class="button" href="/app">"地図アプリ"</a>
                    </p>
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
    options: LeptosOptions,
) -> impl IntoView {
    let title = if user.is_some() {
        "アカウント連携"
    } else {
        "ログイン"
    };
    let css_path = format!("/app{}", options.css_path());

    view! {
        <html lang="ja">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <title>{title}</title>
                <link rel="stylesheet" href=css_path/>
                <HydrationScripts options=options.clone() islands=true root="/app"/>
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
                <AdminNavigation role=user.role/>
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
fn AdminNavigation(role: i64) -> impl IntoView {
    if role != 0 {
        return ().into_any();
    }

    view! {
        <section>
            <h2>"Navigation"</h2>
            <p>
                <a class="button secondary" href="/admin">"管理画面"</a>
            </p>
        </section>
    }
    .into_any()
}

#[component]
fn OptionalAccountDeleteSection(user: model::user::User, account: AccountContext) -> impl IntoView {
    match account.delete_preview {
        Some(preview) => view! {
            <AccountDeleteSection user=user preview=preview/>
        }
        .into_any(),
        None => ().into_any(),
    }
}

#[component]
fn AccountDeleteSection(
    user: model::user::User,
    preview: model::user::UserDeletePreview,
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
            <AccountDeleteForm user=user/>
        </section>
    }
    .into_any()
}

#[island]
fn AccountDeleteForm(user: model::user::User) -> impl IntoView {
    use shared_api::DeleteMe;

    let delete_action = ServerAction::<DeleteMe>::new();
    let nickname = RwSignal::new(String::new());
    let confirm = RwSignal::new(false);
    let error = RwSignal::new(None::<String>);
    let placeholder = user.nickname.clone();

    #[cfg(feature = "hydrate")]
    Effect::new(move |_| match delete_action.value().get() {
        Some(Ok(Ok(_))) => {
            leptos::task::spawn_local(async {
                logout_and_redirect().await;
            });
        }
        Some(Ok(Err(kind))) => {
            error.set(Some(kind.to_string()));
        }
        Some(Err(_)) => {
            error.set(Some("アカウント削除に失敗しました。".to_string()));
        }
        None => {}
    });

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        error.set(None);

        let nickname_confirm = nickname.get().trim().to_owned();
        if nickname_confirm.is_empty() {
            error.set(Some("ニックネームを入力してください。".to_string()));
            return;
        }
        if !confirm.get() {
            error.set(Some("削除の確認にチェックを入れてください。".to_string()));
            return;
        }

        delete_action.dispatch(DeleteMe {
            nickname_confirm,
            confirm_delete: confirm.get(),
        });
    };

    view! {
        <form class="delete-account-form" on:submit=on_submit>
            <label class="delete-confirm-label" for="nickname_confirm">
                "確認のためニックネームを入力してください"
            </label>
            <input
                id="nickname_confirm"
                name="nickname_confirm"
                type="text"
                bind:value=nickname
                autocomplete="off"
                required=true
                placeholder=placeholder
            />
            <label class="delete-confirm-checkbox">
                <input type="checkbox" name="confirm_delete" value="yes" bind:checked=confirm required=true/>
                "削除は取り消せず、同じ OAuth で再ログインすると新しいアカウントになることを理解しました"
            </label>
            {move || error.get().map(|message| view! {
                <p class="delete-error" role="alert">{message}</p>
            })}
            <button class="danger" type="submit" disabled=move || delete_action.pending().get()>
                {move || if delete_action.pending().get() {
                    "削除中..."
                } else {
                    "アカウントを削除"
                }}
            </button>
        </form>
    }
}

#[cfg(feature = "hydrate")]
async fn logout_and_redirect() {
    use leptos::__reexports::wasm_bindgen_futures::JsFuture;
    use leptos::wasm_bindgen::JsCast;

    let window = leptos::web_sys::window().expect("window");
    let init = leptos::web_sys::RequestInit::new();
    init.set_method("POST");
    init.set_credentials(leptos::web_sys::RequestCredentials::Include);

    let request =
        leptos::web_sys::Request::new_with_str_and_init("/logout", &init).expect("logout request");
    let response = JsFuture::from(window.fetch_with_request(&request))
        .await
        .expect("logout fetch");
    let _: leptos::web_sys::Response = response.dyn_into().expect("logout response");
    window.location().set_href("/").expect("redirect home");
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
