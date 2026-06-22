variable "project_id" {
  description = "Google CloudプロジェクトのプロジェクトID"
  type        = string
}

variable "region" {
  description = "デプロイするリージョン"
  type        = string
  default     = "asia-northeast1"
}

variable "service_name" {
  description = "Cloud Runサービスの名前"
  type        = string
  default     = "litestream-sandbox"
}

variable "container_image" {
  description = "デプロイするコンテナイメージのURL"
  type        = string
}

variable "facebook_client_id" {
  description = "Facebook OAuth client ID"
  type        = string
}

variable "github_client_id" {
  description = "GitHub OAuth client ID"
  type        = string
}

variable "terraform_state_bucket" {
  description = "Terraform remote state bucket"
  type        = string
  default     = "river-duxca-prod-terraform-state"
}

variable "storage_state_prefix" {
  description = "Terraform remote state prefix for terraform_gcp_storage"
  type        = string
  default     = "river.duxca.com/gcp_storage"
}
