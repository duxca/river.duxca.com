# Service Account for Cloud Run
resource "google_service_account" "river_container" {
  account_id   = "river-container"
  display_name = "River Container Service Account"
  description  = "Service account for the river application running on Cloud Run"
}
