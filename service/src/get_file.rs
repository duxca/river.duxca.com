#[tracing::instrument(level = "trace", skip(db, gcs))]
pub async fn get_file(
    db: &sqlx::Pool<sqlx::Sqlite>,
    gcs: &google_cloud_storage::client::Client,
    gcs_bucket_name: &str,
    file_id: i64,
) -> Result<Option<(String, bytes::Bytes)>, anyhow::Error> {
    // データベースから画像情報を取得
    let mut conn = db.acquire().await?;
    let file = db::files::get_file(&mut conn, file_id).await?;
    let Some(file) = file else {
        return Ok(None);
    };
    // GCSから画像データを取得
    let res = google_cloud_storage::http::objects::get::GetObjectRequest {
        bucket: gcs_bucket_name.to_string(),
        object: file.gcs_path.clone(),
        ..Default::default()
    };
    let range = google_cloud_storage::http::objects::download::Range::default();
    let data = gcs.download_object(&res, &range).await?;
    Ok(Some((file.content_type, bytes::Bytes::from(data))))
}
