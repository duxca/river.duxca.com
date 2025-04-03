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
    let Some(content_type) = field.content_type() else {
        return Ok((axum::http::StatusCode::BAD_REQUEST, "invalid content-type").into_response());
    };
    if content_type != "image/jpeg" {
        return Ok((axum::http::StatusCode::BAD_REQUEST, "invalid content-type").into_response());
    }
    // TODO: async steram にする
    let data = field.bytes().await?;
    // image かどうかチェック
    // TODO: 画像のサイズを縮小
    // TODO: サムネイル作成
    let gcs_path = format!("images/{}", uuid::Uuid::new_v4());
    let req = google_cloud_storage::http::objects::upload::UploadObjectRequest {
        bucket: st.config.gcs_bucket_name.clone(),
        ..Default::default()
    };
    // アップロードでトランザクションをとる
    // TODO: ロックの種類最適化
    let mut conn = st.db.begin().await?;
    // TODO: このユーザのストレージ使用量をチェック
    // データベースに登録
    let image_id = db::files::create_file(
        &mut conn,
        user.user_id,
        "image/jpeg",
        data.len() as i64,
        &gcs_path,
    )
    .await?;
    let media = google_cloud_storage::http::objects::upload::Media::new(gcs_path);
    let upload_type = google_cloud_storage::http::objects::upload::UploadType::Simple(media);
    // GCSにアップロード
    let google_cloud_storage::http::objects::Object { .. } =
        st.gcs.upload_object(&req, data, &upload_type).await?;
    // アップロード終了で commit
    conn.commit().await?;
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
    // データベースから画像情報を取得
    let mut conn = st.db.acquire().await?;
    let image = db::files::get_file(&mut conn, image_id).await?;
    let Some(image) = image else {
        return Ok((axum::http::StatusCode::NOT_FOUND, "404 not found").into_response());
    };
    // JPG 以外は404
    if image.content_type != "image/jpeg" {
        return Ok((axum::http::StatusCode::NOT_FOUND, "404 not found").into_response());
    }
    // GCSから画像データを取得
    let res = google_cloud_storage::http::objects::get::GetObjectRequest {
        bucket: st.config.gcs_bucket_name.clone(),
        object: image.gcs_path.clone(),
        ..Default::default()
    };
    let range = google_cloud_storage::http::objects::download::Range::default();
    // TODO: async stream にする
    let data = st.gcs.download_object(&res, &range).await?;
    let header = axum::http::HeaderMap::from_iter([(
        axum::http::header::CONTENT_TYPE,
        axum::http::header::HeaderValue::from_str("image/jpeg").unwrap(),
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
    // データベースから画像情報を取得
    let mut conn = st.db.begin().await?;
    let image = db::files::get_file(&mut conn, image_id).await?;
    let Some(image) = image else {
        // TODO: ok にする
        return Ok((axum::http::StatusCode::NOT_FOUND, "404 not found").into_response());
    };
    // ユーザーが一致しない場合は403
    if image.user_id != user.user_id {
        return Ok((axum::http::StatusCode::FORBIDDEN, "403 forbidden").into_response());
    }
    // GCSから画像データを削除
    let req = google_cloud_storage::http::objects::delete::DeleteObjectRequest {
        bucket: st.config.gcs_bucket_name.clone(),
        object: image.gcs_path.clone(),
        ..Default::default()
    };
    st.gcs.delete_object(&req).await?;
    // データベースから画像情報を削除
    db::files::delete_file(&mut conn, image_id).await?;
    conn.commit().await?;
    Ok(axum::http::StatusCode::NO_CONTENT.into_response())
}
