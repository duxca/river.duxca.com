/// POST /image
#[tracing::instrument(level = "trace", skip(auth_session, st, multipart))]
pub async fn upload_image(
    auth_session: axum_login::AuthSession<crate::web::login::Backend>,
    st: axum::extract::State<crate::web::State>,
    mut multipart: axum::extract::Multipart,
) -> anyhow::Result<axum::response::Response, crate::web::Ise> {
    use axum::response::IntoResponse;
    // ユーザー認証チェック
    let user = auth_session.user;
    let Some(user) = user else {
        return Ok((axum::http::StatusCode::UNAUTHORIZED, "401").into_response());
    };
    // フィールド名は問わず、最初のフィールドを取得
    let Some(field) = multipart.next_field().await? else {
        return Ok((axum::http::StatusCode::BAD_REQUEST, "invalid body").into_response());
    };
    let content_type = field.content_type().map(|s| s.to_string());
    let Some(content_type) = content_type else {
        return Ok((axum::http::StatusCode::BAD_REQUEST, "invalid content-type").into_response());
    };
    if content_type != "image/jpeg" {
        return Ok((axum::http::StatusCode::BAD_REQUEST, "invalid content-type").into_response());
    }
    // TODO: async steram にする
    let data = field.bytes().await?;
    let image_id = service::create_file::create_file(
        &st.db,
        &st.gcs,
        &st.config.gcs_bucket_name,
        user.user_id,
        &content_type,
        data,
    )
    .await?;
    let json = serde_json::json!({
        "image_id": image_id
    });
    Ok(axum::response::Json(json).into_response())
}

// TODO:サムネイル用の URL も実装
/// GET /image/:image_id
#[tracing::instrument(level = "trace", skip(st))]
pub async fn get_image(
    axum::extract::Path(image_id): axum::extract::Path<i64>,
    st: axum::extract::State<crate::web::State>,
) -> anyhow::Result<axum::response::Response, crate::web::Ise> {
    use axum::response::IntoResponse;
    let result = service::get_file::get_file(
        &st.db,
        &st.gcs,
        &st.config.gcs_bucket_name,
        image_id,
    )
    .await?;
    let Some((content_type, data)) = result else {
        return Ok((axum::http::StatusCode::NOT_FOUND, "404 not found").into_response());
    };
    // JPG 以外は404
    if content_type != "image/jpeg" {
        return Ok((axum::http::StatusCode::NOT_FOUND, "404 not found").into_response());
    }
    let header = axum::http::HeaderMap::from_iter([(
        axum::http::header::CONTENT_TYPE,
        axum::http::header::HeaderValue::from_str(&content_type).unwrap(),
    )]);
    let body = axum::body::Body::from(data);
    Ok((header, body).into_response())
}

/// DELETE /image/:image_id
#[tracing::instrument(level = "trace", skip(auth_session, st))]
pub async fn delete_image(
    auth_session: axum_login::AuthSession<crate::web::login::Backend>,
    st: axum::extract::State<crate::web::State>,
    axum::extract::Path(image_id): axum::extract::Path<i64>,
) -> anyhow::Result<axum::response::Response, crate::web::Ise> {
    use axum::response::IntoResponse;
    // ユーザー認証チェック
    let user = auth_session.user;
    let Some(user) = user else {
        return Ok((axum::http::StatusCode::UNAUTHORIZED, "401").into_response());
    };
    let result = service::delete_file::delete_file(
        &st.db,
        &st.gcs,
        &st.config.gcs_bucket_name,
        user.user_id,
        image_id,
    )
    .await?;
    if !result {
        return Ok((axum::http::StatusCode::NOT_FOUND, "404 not found").into_response());
    }
    Ok(axum::http::StatusCode::NO_CONTENT.into_response())
}
