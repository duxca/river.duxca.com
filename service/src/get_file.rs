#[tracing::instrument(level = "trace", skip(db, gcs))]
pub async fn get_file(
    db: &sqlx::Pool<sqlx::Sqlite>,
    gcs: &google_cloud_storage::client::Storage,
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
    let bucket = format!("projects/_/buckets/{gcs_bucket_name}");
    let mut res = gcs
        .read_object(bucket, file.gcs_path.clone())
        .send()
        .await?;
    let mut data = bytes::BytesMut::new();
    while let Some(chunk) = res.next().await.transpose()? {
        data.extend_from_slice(&chunk);
    }
    Ok(Some((file.content_type, data.freeze())))
}
