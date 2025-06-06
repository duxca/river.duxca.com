# Google Cloudプロバイダーの設定
provider "google" {
  project = var.project_id
  region  = var.region
}

# Enable required APIs
resource "google_project_service" "cloud_run_api" {
  service = "run.googleapis.com"
}

resource "google_project_service" "secret_manager_api" {
  service = "secretmanager.googleapis.com"
}

resource "google_project_service" "artifact_registry_api" {
  service = "artifactregistry.googleapis.com"
}

resource "google_project_service" "storage_api" {
  service = "storage.googleapis.com"
}

# Cloud Runサービスのデプロイ
# Cloud Runサービスの設定
# サービス名: litestream-sandbox
# リージョン: asia-northeast1
# CPU: 1コア
# メモリ: 256Mi
# タイムアウト: 3秒
# 同時実行数: 128
# 最大インスタンス数: 1
# 最小インスタンス数: 0
# CPUスロットリング: 有効
# CPUブースト: 無効
# 実行環境: gen1
resource "google_cloud_run_service" "litestream_sandbox" {
  name     = "litestream-sandbox"
  location = "asia-northeast1"
  
  lifecycle {
    ignore_changes = [
      template[0].spec[0].containers[0].resources[0].limits,
      autogenerate_revision_name,
      template[0].spec[0].container_concurrency,
      template[0].spec[0].timeout_seconds,
      template[0].metadata[0].annotations
    ]
  }

  template {
    spec {
      container_concurrency = 128
      timeout_seconds       = 3
      
      containers {
        image = var.container_image
        
        resources {
          limits = {
            cpu    = "1"
            memory = "256Mi"
          }
        }

        # シークレットを環境変数として設定
        env {
          name = "FACEBOOK_CLIENT_ID"
          value_from {
            secret_key_ref {
              name = "FACEBOOK_CLIENT_ID"
              key  = "1"
            }
          }
        }

        env {
          name = "FACEBOOK_CLIENT_SECRET"
          value_from {
            secret_key_ref {
              name = "FACEBOOK_CLIENT_SECRET"
              key  = "1"
            }
          }
        }

        env {
          name = "GITHUB_CLIENT_ID"
          value_from {
            secret_key_ref {
              name = "GITHUB_CLIENT_ID"
              key  = "1"
            }
          }
        }

        env {
          name = "GITHUB_CLIENT_SECRET"
          value_from {
            secret_key_ref {
              name = "GITHUB_CLIENT_SECRET"
              key  = "1"
            }
          }
        }

        env {
          name = "TWITTER_CLIENT_ID"
          value_from {
            secret_key_ref {
              name = "TWITTER_CLIENT_ID"
              key  = "1"
            }
          }
        }

        env {
          name = "TWITTER_CLIENT_SECRET"
          value_from {
            secret_key_ref {
              name = "TWITTER_CLIENT_SECRET"
              key  = "1"
            }
          }
        }

        env {
          name = "KEY_JSON"
          value_from {
            secret_key_ref {
              name = "GOOGLE_APPLICATION_CREDENTIALS"
              key  = "3"
            }
          }
        }
      }

      service_account_name = google_service_account.river_container.email
    }

    metadata {
      annotations = {
        "autoscaling.knative.dev/maxScale"         = "1"
        "autoscaling.knative.dev/minScale"         = "0"
        "run.googleapis.com/cpu-throttling"        = "true"
        "run.googleapis.com/cpu-boost"             = "false"
        "run.googleapis.com/execution-environment" = "gen1"
        "run.googleapis.com/timeout"               = "3s"
        "run.googleapis.com/container-concurrency" = "128"
      }
    }
  }

  traffic {
    percent         = 100
    latest_revision = true
  }

  autogenerate_revision_name = false
}

# Cloud Runサービスに対する認証なしアクセスの許可
resource "google_cloud_run_service_iam_member" "public_access" {
  service  = google_cloud_run_service.litestream_sandbox.name
  location = google_cloud_run_service.litestream_sandbox.location
  role     = "roles/run.invoker"
  member   = "allUsers"
}

# GCSバケットへのアクセス権を付与
resource "google_storage_bucket_iam_member" "bucket_object_admin" {
  bucket = google_storage_bucket.litestream_bucket.name
  role   = "roles/storage.objectAdmin"
  member = "serviceAccount:${google_service_account.river_container.email}"
}

resource "google_storage_bucket_iam_member" "bucket_legacy_reader" {
  bucket = google_storage_bucket.litestream_bucket.name
  role   = "roles/storage.legacyBucketReader"
  member = "serviceAccount:${google_service_account.river_container.email}"
}

resource "google_storage_bucket_iam_member" "bucket_legacy_writer" {
  bucket = google_storage_bucket.litestream_bucket.name
  role   = "roles/storage.legacyBucketWriter"
  member = "serviceAccount:${google_service_account.river_container.email}"
}