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

resource "google_secret_manager_secret" "twitter_client_id" {
  secret_id = "TWITTER_CLIENT_ID"

  replication {
    auto {}
  }
}

resource "google_secret_manager_secret" "twitter_client_secret" {
  secret_id = "TWITTER_CLIENT_SECRET"

  replication {
    auto {}
  }
}

resource "google_secret_manager_secret" "google_application_credentials" {
  secret_id = "GOOGLE_APPLICATION_CREDENTIALS"

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

resource "google_secret_manager_secret_iam_member" "twitter_client_id_access" {
  secret_id = google_secret_manager_secret.twitter_client_id.secret_id
  role      = "roles/secretmanager.secretAccessor"
  member    = "serviceAccount:${google_service_account.river_container.email}"
}

resource "google_secret_manager_secret_iam_member" "twitter_client_secret_access" {
  secret_id = google_secret_manager_secret.twitter_client_secret.secret_id
  role      = "roles/secretmanager.secretAccessor"
  member    = "serviceAccount:${google_service_account.river_container.email}"
}

resource "google_secret_manager_secret_iam_member" "google_application_credentials_access" {
  secret_id = google_secret_manager_secret.google_application_credentials.secret_id
  role      = "roles/secretmanager.secretAccessor"
  member    = "serviceAccount:${google_service_account.river_container.email}"
}