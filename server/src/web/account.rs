const ACCOUNT_CSRF_TOKEN_KEY: &str = "account.csrf-token";

pub async fn account_csrf_token(session: &tower_sessions::Session) -> Result<String, crate::web::Ise> {
    use anyhow::Context;

    if let Some(token) = session
        .get::<String>(ACCOUNT_CSRF_TOKEN_KEY)
        .await
        .context("Failed to get account CSRF token from session")?
    {
        return Ok(token);
    }

    let token = oauth2::CsrfToken::new_random().secret().to_string();
    session
        .insert(ACCOUNT_CSRF_TOKEN_KEY, &token)
        .await
        .context("Failed to insert account CSRF token into session")?;
    session
        .save()
        .await
        .context("Failed to save session after account CSRF token insertion")?;
    Ok(token)
}

async fn validate_account_csrf_token(
    session: &tower_sessions::Session,
    csrf_token: &str,
) -> Result<bool, crate::web::Ise> {
    use anyhow::Context;

    let saved_token = session
        .get::<String>(ACCOUNT_CSRF_TOKEN_KEY)
        .await
        .context("Failed to get account CSRF token from session")?;
    Ok(saved_token.as_deref() == Some(csrf_token))
}

#[derive(Debug, serde::Deserialize)]
pub struct DeleteAccountForm {
    pub csrf_token: String,
    pub nickname_confirm: String,
    pub confirm_delete: Option<String>,
}

/// POST /account/delete
#[tracing::instrument(level = "trace", skip(auth_session, session, st))]
pub async fn delete_account(
    mut auth_session: axum_login::AuthSession<crate::web::login::Backend>,
    session: tower_sessions::Session,
    axum::extract::State(ref st): axum::extract::State<crate::web::State>,
    axum_extra::extract::Form(form): axum_extra::extract::Form<DeleteAccountForm>,
) -> Result<impl axum::response::IntoResponse, crate::web::Ise> {
    use axum::response::IntoResponse;

    let Some(ref user) = auth_session.user else {
        return Ok((axum::http::StatusCode::UNAUTHORIZED, "401").into_response());
    };
    if !validate_account_csrf_token(&session, &form.csrf_token).await? {
        return Ok((axum::http::StatusCode::BAD_REQUEST, "invalid csrf token").into_response());
    }
    if form.nickname_confirm != user.nickname {
        return Ok((
            axum::http::StatusCode::BAD_REQUEST,
            "nickname confirmation does not match",
        )
            .into_response());
    }
    if form.confirm_delete.as_deref() != Some("yes") {
        return Ok((
            axum::http::StatusCode::BAD_REQUEST,
            "delete confirmation is required",
        )
            .into_response());
    }

    let res = service::handler(
        &st.db,
        user,
        model::api::delete_me::Request {}.into(),
    )
    .await?;
    match res {
        model::api::Response::DeleteMe(_) => {}
        model::api::Response::Error(model::api::ErrorKind::PermissionDenied) => {
            return Ok((
                axum::http::StatusCode::FORBIDDEN,
                "admin accounts cannot be deleted",
            )
                .into_response());
        }
        _ => {
            return Err(crate::web::Ise(anyhow::anyhow!(
                "unexpected delete_me response: {res:?}"
            )));
        }
    }

    auth_session.logout().await?;
    session.flush().await?;
    Ok(axum::response::Redirect::to("/").into_response())
}