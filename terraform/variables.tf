# 変数定義
variable "project_id" {
  description = "Google CloudプロジェクトのプロジェクトID"
  type        = string
  default     = "duxca-298210"
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