# Google Cloudプロバイダーの設定
provider "google" {
  project = var.project_id
  region  = var.region
}

# 特定のGCSバケットへのアクセス権を付与
resource "google_storage_bucket_iam_member" "bucket_object_admin" {
  bucket = "duxca-litestream-sandbox"
  role   = "roles/storage.objectAdmin"
  member = "serviceAccount:${var.service_account_email}"
}

resource "google_storage_bucket_iam_member" "bucket_legacy_reader" {
  bucket = "duxca-litestream-sandbox"
  role   = "roles/storage.legacyBucketReader"
  member = "serviceAccount:${var.service_account_email}"
}

resource "google_storage_bucket_iam_member" "bucket_legacy_writer" {
  bucket = "duxca-litestream-sandbox"
  role   = "roles/storage.legacyBucketWriter"
  member = "serviceAccount:${var.service_account_email}"
}