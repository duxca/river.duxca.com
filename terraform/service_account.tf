# Service Account for Cloud Run
resource "google_service_account" "river_container" {
  account_id   = "river-container"
  display_name = "River Container Service Account"
  description  = "Service account for the river application running on Cloud Run"
}

# Service Account Key (for local development)
resource "google_service_account_key" "river_container_key" {
  service_account_id = google_service_account.river_container.name
}