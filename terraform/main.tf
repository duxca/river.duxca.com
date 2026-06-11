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
# タイムアウト: 60秒
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
      autogenerate_revision_name
    ]
  }

  template {
    spec {
      container_concurrency = 128
      # sqlite の busy_timeout よりは大きくする必要ある:w
      timeout_seconds = 60

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
