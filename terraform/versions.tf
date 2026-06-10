terraform {
  required_version = ">= 1.0.0"

  backend "gcs" {
    bucket = "duxca-terraform-state"
    prefix = "river.duxca.com"
  }

  required_providers {
    google = {
      source  = "hashicorp/google"
      version = ">= 6.0.0, < 7.0.0"
    }
    cloudflare = {
      source  = "cloudflare/cloudflare"
      version = "~> 5.19.1"
    }
  }
}
