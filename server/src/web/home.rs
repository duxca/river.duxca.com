/// GET /
#[tracing::instrument(level = "trace", skip(auth_session, st))]
pub async fn home(
    auth_session: axum_login::AuthSession<crate::web::login::Backend>,
    axum::extract::State(ref st): axum::extract::State<crate::web::State>,
) -> Result<impl axum::response::IntoResponse, crate::web::Ise> {
    use askama::Template;
    use axum::response::IntoResponse;

    let user = auth_session.user;
    let auths = if let Some(user) = user.as_ref() {
        let mut conn = st.db.acquire().await?;
        db::user::get_user_auths(&mut conn, user.user_id).await?
    } else {
        vec![]
    };

    #[derive(Debug, askama::Template)]
    #[template(
        source = r#"<!doctype html>
<html lang="ja">
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>river.duxca.com</title>
    <style>
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
        margin: 0 0 8px;
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
        margin: 24px 0;
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
    </style>
  </head>
  <body>
    <main>
      <h1>river.duxca.com</h1>
      <p>川下り地図アプリのサーバは動いています。現在は Yew UI を切り離して、ログイン確認用の簡易ページを表示しています。</p>
      {% match user %}
      {% when Some with (user) %}
      <dl>
        <dt>Status</dt>
        <dd>ログイン済み</dd>
        <dt>User ID</dt>
        <dd><code>{{ user.user_id }}</code></dd>
        <dt>Nickname</dt>
        <dd>{{ user.nickname }}</dd>
        <dt>Role</dt>
        <dd><code>{{ user.role }}</code></dd>
      </dl>
      <section>
        <h2>Connected accounts</h2>
        <div class="provider">
          <div>
            <div class="provider-name">GitHub</div>
            {% if let Some(auth) = github %}
            <div class="provider-id">connected: {{ auth.identifier }}</div>
            {% else %}
            <div class="provider-id">not connected</div>
            {% endif %}
          </div>
        </div>
        <div class="provider">
          <div>
            <div class="provider-name">Facebook</div>
            {% if let Some(auth) = facebook %}
            <div class="provider-id">connected: {{ auth.identifier }}</div>
            {% else %}
            <div class="provider-id">not connected</div>
            {% endif %}
          </div>
        </div>
        <a class="button secondary" href="/login">Manage connections</a>
      </section>
      <form method="post" action="/logout">
        <button class="secondary" type="submit">Logout</button>
      </form>
      {% when None %}
      <dl>
        <dt>Status</dt>
        <dd>未ログイン</dd>
      </dl>
      <form method="post" action="/login/github">
        <button type="submit">Login with GitHub</button>
      </form>
      <a class="button secondary" href="/login">Provider status</a>
      {% endmatch %}
      <p><a href="/version">version</a></p>
    </main>
  </body>
</html>"#,
        ext = "html"
    )]
    struct Tmpl {
        user: Option<model::user::User>,
        github: Option<model::user::UserAuth>,
        facebook: Option<model::user::UserAuth>,
    }

    Ok(axum::response::Html(
        Tmpl {
            user,
            github: auths
                .iter()
                .find(|a| a.identity_type == 0)
                .map(ToOwned::to_owned),
            facebook: auths
                .iter()
                .find(|a| a.identity_type == 1)
                .map(ToOwned::to_owned),
        }
        .render()?,
    )
    .into_response())
}
