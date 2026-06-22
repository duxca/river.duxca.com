terraform {
  required_version = ">= 1.0.0"

  backend "gcs" {
    bucket = "river-duxca-prod-terraform-state"
    prefix = "river.duxca.com/cf"
  }

  required_providers {
    cloudflare = {
      source  = "cloudflare/cloudflare"
      version = "~> 5.19.1"
    }
  }
}
