# Artifact Registry for Docker images
resource "google_artifact_registry_repository" "docker_repo" {
  location      = var.region
  repository_id = var.docker_registry
  description   = "Docker repository for river application"
  format        = "DOCKER"

}

# IAM permission for service account to pull images
resource "google_artifact_registry_repository_iam_member" "docker_repo_reader" {
  location   = google_artifact_registry_repository.docker_repo.location
  repository = google_artifact_registry_repository.docker_repo.name
  role       = "roles/artifactregistry.reader"
  member     = "serviceAccount:${google_service_account.river_container.email}"
}