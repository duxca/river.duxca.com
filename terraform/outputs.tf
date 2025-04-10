# 出力値の定義
output "cloud_run_url" {
  description = "Cloud Runサービスの公開URL"
  value       = google_cloud_run_service.litestream_sandbox.status[0].url
}

output "cloud_run_service_name" {
  description = "Cloud Runサービスの名前"
  value       = google_cloud_run_service.litestream_sandbox.name
}

output "cloud_run_service_location" {
  description = "Cloud Runサービスのリージョン"
  value       = google_cloud_run_service.litestream_sandbox.location
}