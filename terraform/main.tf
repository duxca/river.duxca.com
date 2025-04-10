# Google Cloudプロバイダーの設定
provider "google" {
  project = "duxca-298210"
  region  = "asia-northeast1"
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

  template {
    spec {
      containers {
        image = "asia-northeast1-docker.pkg.dev/duxca-298210/cloud-run-source-deploy/litestream-sandbox:latest"
        
        resources {
          limits = {
            cpu    = "1000m"
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

      service_account_name = "river-container@duxca-298210.iam.gserviceaccount.com"
    }

    metadata {
      annotations = {
        "autoscaling.knative.dev/maxScale"      = "1"
        "autoscaling.knative.dev/minScale"      = "0"
        "run.googleapis.com/cpu-throttling"     = "true"
        "run.googleapis.com/cpu-boost"          = "false"
        "run.googleapis.com/execution-environment" = "gen1"
        "run.googleapis.com/container-concurrency" = "128"
        "run.googleapis.com/timeout"            = "3s"
      }
    }
  }

  traffic {
    percent         = 100
    latest_revision = true
  }

  autogenerate_revision_name = true
}

# Cloud Runサービスに対する認証なしアクセスの許可
resource "google_cloud_run_service_iam_member" "public_access" {
  service  = google_cloud_run_service.litestream_sandbox.name
  location = google_cloud_run_service.litestream_sandbox.location
  role     = "roles/run.invoker"
  member   = "allUsers"
}

# 特定のGCSバケットへのアクセス権を付与
resource "google_storage_bucket_iam_member" "bucket_object_admin" {
  bucket = "duxca-litestream-sandbox"
  role   = "roles/storage.objectAdmin"
  member = "serviceAccount:river-container@duxca-298210.iam.gserviceaccount.com"
}

resource "google_storage_bucket_iam_member" "bucket_legacy_reader" {
  bucket = "duxca-litestream-sandbox"
  role   = "roles/storage.legacyBucketReader"
  member = "serviceAccount:river-container@duxca-298210.iam.gserviceaccount.com"
}

resource "google_storage_bucket_iam_member" "bucket_legacy_writer" {
  bucket = "duxca-litestream-sandbox"
  role   = "roles/storage.legacyBucketWriter"
  member = "serviceAccount:river-container@duxca-298210.iam.gserviceaccount.com"
}