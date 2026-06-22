provider "google" {
  project = var.project_id
  region  = var.region
}

resource "google_cloud_run_service" "litestream_sandbox" {
  name     = var.service_name
  location = var.region

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

        env {
          name  = "LITESTREAM_BUCKET"
          value = data.terraform_remote_state.storage.outputs.litestream_bucket_name
        }

        resources {
          limits = {
            cpu    = "1"
            memory = "256Mi"
          }
        }

        # シークレットを環境変数として設定
        env {
          name  = "FACEBOOK_CLIENT_ID"
          value = var.facebook_client_id
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
          name  = "GITHUB_CLIENT_ID"
          value = var.github_client_id
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

      service_account_name = data.terraform_remote_state.storage.outputs.river_container_service_account_email
    }

    metadata {
      annotations = {
        "autoscaling.knative.dev/maxScale"         = "1"
        "autoscaling.knative.dev/minScale"         = "0"
        "run.googleapis.com/cpu-throttling"        = "true"
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

resource "google_cloud_run_service_iam_member" "public_access" {
  service  = google_cloud_run_service.litestream_sandbox.name
  location = google_cloud_run_service.litestream_sandbox.location
  role     = "roles/run.invoker"
  member   = "allUsers"
}
