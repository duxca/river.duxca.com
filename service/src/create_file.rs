#[tracing::instrument(level = "trace", skip(db, gcs))]
pub async fn create_file(
    db: &sqlx::Pool<sqlx::Sqlite>,
    gcs: &google_cloud_storage::client::Client,
    gcs_bucket_name: &str,
    user_id: i64,
    content_type: &str,
    data: bytes::Bytes,
) -> Result<i64, anyhow::Error> {
    let gcs_path = format!("files/{}", uuid::Uuid::new_v4());
    let req = google_cloud_storage::http::objects::upload::UploadObjectRequest {
        bucket: gcs_bucket_name.to_string(),
        ..Default::default()
    };
    // アップロードでトランザクションをとる
    let mut conn = db.begin().await?;
    // データベースに登録
    let file_id = db::files::create_file(
        &mut conn,
        user_id,
        content_type,
        data.len() as i64,
        &gcs_path,
    )
    .await?;
    let media = google_cloud_storage::http::objects::upload::Media::new(gcs_path);
    let upload_type = google_cloud_storage::http::objects::upload::UploadType::Simple(media);
    // GCSにアップロード
    let _object = gcs.upload_object(&req, data, &upload_type).await?;
    // アップロード終了で commit
    conn.commit().await?;
    Ok(file_id)
}