resource "google_cloud_run_domain_mapping" "river" {
  name     = "river.duxca.com"
  location = var.region

  metadata {
    namespace = var.project_number
  }

  spec {
    route_name       = google_cloud_run_service.litestream_sandbox.name
    certificate_mode = "AUTOMATIC"
  }
}
