variable "project_id" {
  description = "Google Cloud project ID"
  type        = string
}

variable "project_number" {
  description = "Google Cloud project number"
  type        = string
}

variable "github_repositories" {
  description = "GitHub repositories allowed to impersonate the deployer service account from main"
  type        = list(string)
}

variable "workload_identity_pool_id" {
  description = "Workload Identity Pool ID for GitHub Actions"
  type        = string
  default     = "githubaction"
}

variable "workload_identity_provider_id" {
  description = "Workload Identity Provider ID for GitHub Actions"
  type        = string
  default     = "github"
}

variable "deployer_service_account_id" {
  description = "GitHub Actions deployer service account ID"
  type        = string
  default     = "github-action-river"
}

variable "terraform_state_bucket" {
  description = "Terraform state bucket name"
  type        = string
  default     = "river-duxca-prod-terraform-state"
}

variable "app_service_account_email" {
  description = "Cloud Run runtime service account email"
  type        = string
}

variable "region" {
  description = "Region of the Cloud Run service"
  type        = string
  default     = "asia-northeast1"
}

variable "app_service_name" {
  description = "Name of the Cloud Run service"
  type        = string
  default     = "litestream-sandbox"
}

variable "bucket_name" {
  description = "Litestream用のGCSバケット名"
  type        = string
}

variable "litestream_bucket_force_destroy" {
  description = "Litestream用GCSバケット内のオブジェクトごと削除するか"
  type        = bool
  default     = false
}

variable "docker_registry" {
  description = "Docker Artifact Registry リポジトリ名"
  type        = string
  default     = "cloud-run-source-deploy"
}
