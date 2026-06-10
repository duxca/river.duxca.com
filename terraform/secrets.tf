# Secret Manager secrets for OAuth credentials
resource "google_secret_manager_secret" "facebook_client_id" {
  secret_id = "FACEBOOK_CLIENT_ID"

  replication {
    auto {}
  }
}

resource "google_secret_manager_secret" "facebook_client_secret" {
  secret_id = "FACEBOOK_CLIENT_SECRET"

  replication {
    auto {}
  }
}

resource "google_secret_manager_secret" "github_client_id" {
  secret_id = "GITHUB_CLIENT_ID"

  replication {
    auto {}
  }
}

resource "google_secret_manager_secret" "github_client_secret" {
  secret_id = "GITHUB_CLIENT_SECRET"

  replication {
    auto {}
  }
}

# Secret versions would need to be added manually or via separate terraform apply
# with actual values from your OAuth applications

# IAM permissions for service account to access secrets
resource "google_secret_manager_secret_iam_member" "facebook_client_id_access" {
  secret_id = google_secret_manager_secret.facebook_client_id.secret_id
  role      = "roles/secretmanager.secretAccessor"
  member    = "serviceAccount:${google_service_account.river_container.email}"
}

resource "google_secret_manager_secret_iam_member" "facebook_client_secret_access" {
  secret_id = google_secret_manager_secret.facebook_client_secret.secret_id
  role      = "roles/secretmanager.secretAccessor"
  member    = "serviceAccount:${google_service_account.river_container.email}"
}

resource "google_secret_manager_secret_iam_member" "github_client_id_access" {
  secret_id = google_secret_manager_secret.github_client_id.secret_id
  role      = "roles/secretmanager.secretAccessor"
  member    = "serviceAccount:${google_service_account.river_container.email}"
}

resource "google_secret_manager_secret_iam_member" "github_client_secret_access" {
  secret_id = google_secret_manager_secret.github_client_secret.secret_id
  role      = "roles/secretmanager.secretAccessor"
  member    = "serviceAccount:${google_service_account.river_container.email}"
}
