#[tracing::instrument(level = "trace", skip(db, gcs))]
pub async fn create_file(
    db: &sqlx::Pool<sqlx::Sqlite>,
    gcs: &google_cloud_storage::client::Storage,
    gcs_bucket_name: &str,
    user_id: i64,
    content_type: &str,
    data: bytes::Bytes,
) -> Result<i64, anyhow::Error> {
    let gcs_path = format!("files/{}", uuid::Uuid::new_v4());
    let bucket = format!("projects/_/buckets/{gcs_bucket_name}");
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
    // GCSにアップロード
    let _object = gcs
        .write_object(bucket, gcs_path, data)
        .set_content_type(content_type)
        .send_buffered()
        .await?;
    // アップロード終了で commit
    conn.commit().await?;
    Ok(file_id)
}
