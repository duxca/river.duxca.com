# 変数定義
variable "project_id" {
  description = "Google CloudプロジェクトのプロジェクトID"
  type        = string
  default     = "duxca-298210"
}

variable "project_number" {
  description = "Google Cloudプロジェクトのプロジェクト番号"
  type        = string
  default     = "93254674393"
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
  default     = "asia-northeast1-docker.pkg.dev/duxca-298210/cloud-run-source-deploy/litestream-sandbox:latest"
}

variable "service_account_email" {
  description = "サービスで使用するサービスアカウントのメールアドレス"
  type        = string
  default     = "river-container@duxca-298210.iam.gserviceaccount.com"
}

variable "docker_registry" {
  description = "Docker Artifact Registry リポジトリ名"
  type        = string
  default     = "cloud-run-source-deploy"
}

variable "cloudflare_account_id" {
  description = "Cloudflare account ID"
  type        = string
  default     = "688933c4553b4284a2684583893badc9"
}

variable "cloudflare_zone_id" {
  description = "Cloudflare zone ID for duxca.com"
  type        = string
  default     = "3b9a8608fb6557722d27b468e461767e"
}
