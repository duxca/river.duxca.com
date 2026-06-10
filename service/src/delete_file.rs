#[tracing::instrument(level = "trace", skip(db, gcs))]
pub async fn delete_file(
    db: &sqlx::Pool<sqlx::Sqlite>,
    gcs: &google_cloud_storage::client::StorageControl,
    gcs_bucket_name: &str,
    user_id: i64,
    file_id: i64,
) -> Result<bool, anyhow::Error> {
    // データベースから画像情報を取得
    let mut conn = db.begin().await?;
    let file = db::files::get_file(&mut conn, file_id).await?;
    let Some(file) = file else {
        return Ok(false);
    };
    // ユーザーが一致しない場合は403
    if file.user_id != user_id {
        return Ok(false);
    }
    // GCSから画像データを削除
    let bucket = format!("projects/_/buckets/{gcs_bucket_name}");
    gcs.delete_object()
        .set_bucket(bucket)
        .set_object(file.gcs_path.clone())
        .send()
        .await?;
    // データベースから画像情報を削除
    db::files::delete_file(&mut conn, file_id).await?;
    conn.commit().await?;
    Ok(true)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     /// テスト用のGCSエンドポイントを取得
//     fn get_gcs_endpoint() -> String {
//         std::env::var("GCS_ENDPOINT").unwrap_or_else(|_| "http://localhost:4443".to_string())
//     }

//     /// バケットが存在するか確認
//     async fn check_bucket_exists(bucket_name: &str) -> Result<bool, anyhow::Error> {
//         let client = reqwest::Client::new();
//         let endpoint = get_gcs_endpoint();
//         let url = format!("{}/storage/v1/b/{}", endpoint, bucket_name);

//         let response = client.get(url).send().await?;
//         Ok(response.status().is_success())
//     }

//     /// テスト用のバケットを作成
//     async fn create_bucket(bucket_name: &str) -> Result<(), anyhow::Error> {
//         // すでに存在する場合はスキップ
//         if check_bucket_exists(bucket_name).await? {
//             return Ok(());
//         }

//         let client = reqwest::Client::new();
//         let endpoint = get_gcs_endpoint();
//         let url = format!(
//             "{}/storage/v1/b?name={}&project=test-project",
//             endpoint, bucket_name
//         );

//         let response = client
//             .post(url)
//             .header("Content-Length", "0")
//             .send()
//             .await?;

//         if !response.status().is_success() {
//             return Err(anyhow::anyhow!(
//                 "Failed to create bucket: {}",
//                 response.status()
//             ));
//         }

//         Ok(())
//     }

//     async fn setup() -> Result<
//         (
//             sqlx::Pool<sqlx::Sqlite>,
//             google_cloud_storage::client::Client,
//             String,
//         ),
//         anyhow::Error,
//     > {
//         // SQLite の一時データベースを作成
//         let db = sqlx::sqlite::SqlitePoolOptions::new()
//             .max_connections(1)
//             .connect("sqlite::memory:")
//             .await
//             .map_err(|e| anyhow::anyhow!("Failed to create SQLite database: {}", e))?;

//         // マイグレーションを実行
//         sqlx::migrate!("../db/migrations")
//             .run(&db)
//             .await
//             .map_err(|e| anyhow::anyhow!("Failed to run migrations: {}", e))?;

//         // GCS モッククライアントを作成
//         let endpoint = get_gcs_endpoint();
//         let mut config = google_cloud_storage::client::ClientConfig::default();
//         config.storage_endpoint = endpoint.clone();
//         config.service_account_endpoint = endpoint;
//         let gcs = google_cloud_storage::client::Client::new(config);

//         // テスト用のバケット名
//         let bucket_name = format!("test-bucket-{}", uuid::Uuid::new_v4());
//         create_bucket(&bucket_name).await?;

//         Ok((db, gcs, bucket_name))
//     }

//     #[tokio::test]
//     async fn test_upload_get_delete_flow() -> Result<(), anyhow::Error> {
//         let (db, gcs, bucket_name) = setup().await?;

//         // テスト用の画像データ
//         let content_type = "file/jpeg";
//         let data = bytes::Bytes::from_static(b"fake jpg data");
//         let user_id = 1;

//         // アップロード
//         let file_id = create_file(
//             &db,
//             &gcs,
//             bucket_name.clone(),
//             user_id,
//             content_type,
//             data.clone(),
//         )
//         .await?;
//         assert!(file_id > 0);

//         // 取得
//         let result = get_file(&db, &gcs, bucket_name.clone(), file_id).await?;
//         assert!(result.is_some(), "Failed to get uploaded file");
//         let (got_content_type, got_data) = result.unwrap();
//         assert_eq!(got_content_type, content_type);
//         assert_eq!(got_data, data);

//         // 削除
//         let deleted = delete_file(&db, &gcs, bucket_name.clone(), user_id, file_id).await?;
//         assert!(deleted, "Failed to delete file");

//         // 削除後の取得確認
//         let result = get_file(&db, &gcs, bucket_name, file_id).await?;
//         assert!(result.is_none(), "file still exists after deletion");

//         Ok(())
//     }

//     #[tokio::test]
//     async fn test_get_nonexistent_file() -> Result<(), anyhow::Error> {
//         let (db, gcs, bucket_name) = setup().await?;

//         let result = get_file(&db, &gcs, bucket_name, 999).await?;
//         assert!(result.is_none(), "Nonexistent file should return None");

//         Ok(())
//     }

//     #[tokio::test]
//     async fn test_delete_nonexistent_file() -> Result<(), anyhow::Error> {
//         let (db, gcs, bucket_name) = setup().await?;

//         let deleted = delete_file(&db, &gcs, bucket_name, 1, 999).await?;
//         assert!(!deleted, "Deleting nonexistent file should return false");

//         Ok(())
//     }

//     #[tokio::test]
//     async fn test_delete_unauthorized_file() -> Result<(), anyhow::Error> {
//         let (db, gcs, bucket_name) = setup().await?;

//         // ユーザー1でアップロード
//         let content_type = "file/jpeg";
//         let data = bytes::Bytes::from_static(b"fake jpg data");
//         let user_id = 1;
//         let file_id =
//             create_file(&db, &gcs, bucket_name.clone(), user_id, content_type, data).await?;

//         // ユーザー2で削除を試みる
//         let other_user_id = 2;
//         let deleted = delete_file(&db, &gcs, bucket_name, other_user_id, file_id).await?;
//         assert!(!deleted, "Unauthorized deletion should return false");

//         Ok(())
//     }

//     #[tokio::test]
//     async fn test_get_non_jpeg_file() -> Result<(), anyhow::Error> {
//         let (db, gcs, bucket_name) = setup().await?;

//         // PNG画像をアップロード
//         let content_type = "file/png";
//         let data = bytes::Bytes::from_static(b"fake png data");
//         let user_id = 1;
//         let file_id =
//             create_file(&db, &gcs, bucket_name.clone(), user_id, content_type, data).await?;

//         // 取得を試みる（JPG以外は404になる）
//         let result = get_file(&db, &gcs, bucket_name, file_id).await?;
//         assert!(result.is_none(), "Non-JPEG file should return None");

//         Ok(())
//     }
// }
