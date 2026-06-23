# GCS bucket for Litestream database replicas.
resource "google_storage_bucket" "litestream_bucket" {
  name          = var.bucket_name
  location      = var.region
  force_destroy = var.litestream_bucket_force_destroy

  lifecycle {
    prevent_destroy = false
  }

  versioning {
    enabled = true
  }

  lifecycle_rule {
    condition {
      age = 90
    }
    action {
      type = "Delete"
    }
  }

  uniform_bucket_level_access = true

  depends_on = [google_project_service.storage_api]
}

resource "google_storage_bucket_iam_member" "bucket_object_admin" {
  bucket = google_storage_bucket.litestream_bucket.name
  role   = "roles/storage.objectAdmin"
  member = "serviceAccount:${google_service_account.river_container.email}"
}
