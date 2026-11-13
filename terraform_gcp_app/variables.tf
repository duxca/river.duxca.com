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

variable "facebook_client_secret_version" {
  description = "Secret Manager version for FACEBOOK_CLIENT_SECRET"
  type        = string
  default     = "2"
}

variable "github_client_secret_version" {
  description = "Secret Manager version for GITHUB_CLIENT_SECRET"
  type        = string
  default     = "4"
}

variable "terraform_state_bucket" {
  description = "Terraform remote state bucket"
  type        = string
  default     = "river-duxca-prod-terraform-state"
}

variable "foundation_state_prefix" {
  description = "Terraform remote state prefix for terraform_ci"
  type        = string
  default     = "river.duxca.com/ci"
}
