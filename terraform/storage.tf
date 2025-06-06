# GCS Bucket for Litestream backups and file storage
resource "google_storage_bucket" "litestream_bucket" {
  name     = var.bucket_name
  location = var.region
  
  # Prevent accidental deletion
  lifecycle {
    prevent_destroy = true
  }
  
  # Versioning for data protection
  versioning {
    enabled = true
  }
  
  # Lifecycle management
  lifecycle_rule {
    condition {
      age = 90
    }
    action {
      type = "Delete"
    }
  }
  
  # Enable uniform bucket-level access
  uniform_bucket_level_access = true
}