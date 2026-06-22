# Bootstrap configuration to create the Terraform state bucket
# This should be applied first, before configuring the remote backend

terraform {
  required_version = ">= 1.0.0"

  required_providers {
    google = {
      source  = "hashicorp/google"
      version = ">= 6.0.0, < 7.0.0"
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

  force_destroy = false

  lifecycle {
    prevent_destroy = true
  }

  versioning {
    enabled = true
  }

  uniform_bucket_level_access = true
  public_access_prevention    = "enforced"

  lifecycle_rule {
    condition {
      age                = 30
      num_newer_versions = 5
    }
    action {
      type = "Delete"
    }
  }
}

output "terraform_state_bucket" {
  value       = google_storage_bucket.terraform_state.name
  description = "GCS bucket for Terraform state"
}
