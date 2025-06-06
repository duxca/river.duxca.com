terraform {
  required_version = ">= 1.0.0"
  
  backend "gcs" {
    bucket = "duxca-terraform-state"
    prefix = "river.duxca.com"
  }
  
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = ">= 4.0.0, < 5.0.0"
    }
  }
}