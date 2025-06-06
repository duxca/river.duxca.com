# Bootstrap configuration to create the Terraform state bucket
# This should be applied first, before configuring the remote backend

terraform {
  required_version = ">= 1.0.0"
  
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = ">= 4.0.0, < 5.0.0"
    }
  }
}

provider "google" {
  project = var.project_id
  region  = var.region
}

# GCS bucket for Terraform state
resource "google_storage_bucket" "terraform_state" {
  name     = var.terraform_state_bucket
  location = var.region
  
  # Force destroy for easier management
  force_destroy = false
  
  # Prevent accidental deletion
  lifecycle {
    prevent_destroy = true
  }
  
  # Versioning for state file history
  versioning {
    enabled = true
  }
  
  # Enable uniform bucket-level access
  uniform_bucket_level_access = true
  
  # Lifecycle management for old state versions
  lifecycle_rule {
    condition {
      age                   = 30
      num_newer_versions    = 5
    }
    action {
      type = "Delete"
    }
  }
}

# Output the bucket name for reference
output "terraform_state_bucket" {
  value       = google_storage_bucket.terraform_state.name
  description = "GCS bucket for Terraform state"
}