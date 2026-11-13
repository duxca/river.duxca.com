provider "google" {
  project = var.project_id
}

locals {
  deployer_service_account_email = "${var.deployer_service_account_id}@${var.project_id}.iam.gserviceaccount.com"
  deployer_member                = "serviceAccount:${local.deployer_service_account_email}"
  github_main_ref_principal      = "principalSet://iam.googleapis.com/projects/${var.project_number}/locations/global/workloadIdentityPools/${var.workload_identity_pool_id}/attribute.ref/refs/heads/main"
  github_repository_condition    = join(" || ", [for repository in var.github_repositories : "attribute.repository == '${repository}'"])

  project_roles = toset([
    "roles/artifactregistry.writer",
    "roles/run.developer",
  ])
}

resource "google_service_account" "deployer" {
  account_id   = var.deployer_service_account_id
  display_name = "GitHub Actions River deployer"
}

resource "google_iam_workload_identity_pool" "github_actions" {
  workload_identity_pool_id = var.workload_identity_pool_id
  display_name              = "GitHub Actions"
}

resource "google_iam_workload_identity_pool_provider" "github" {
  workload_identity_pool_id          = google_iam_workload_identity_pool.github_actions.workload_identity_pool_id
  workload_identity_pool_provider_id = var.workload_identity_provider_id
  display_name                       = "GitHub"

  attribute_mapping = {
    "google.subject"       = "assertion.sub"
    "attribute.repository" = "assertion.repository"
    "attribute.actor"      = "assertion.actor"
    "attribute.ref"        = "assertion.ref"
  }

  attribute_condition = "(${local.github_repository_condition}) && attribute.ref == 'refs/heads/main'"

  oidc {
    issuer_uri = "https://token.actions.githubusercontent.com"
  }
}

resource "google_service_account_iam_member" "github_actions_impersonation" {
  service_account_id = google_service_account.deployer.name
  role               = "roles/iam.workloadIdentityUser"
  member             = local.github_main_ref_principal
}

resource "google_project_iam_member" "deployer_project_roles" {
  for_each = local.project_roles

  project = var.project_id
  role    = each.value
  member  = local.deployer_member
}

resource "google_storage_bucket_iam_member" "terraform_state_object_admin" {
  bucket = var.terraform_state_bucket
  role   = "roles/storage.objectAdmin"
  member = local.deployer_member
}

resource "google_service_account_iam_member" "app_service_account_user" {
  service_account_id = "projects/${var.project_id}/serviceAccounts/${var.app_service_account_email}"
  role               = "roles/iam.serviceAccountUser"
  member             = local.deployer_member
}

# The deploy workflow verifies the expected secret versions by accessing
# them, so the deployer gets payload access to the two app secrets only.
resource "google_secret_manager_secret_iam_member" "deployer_secret_access" {
  for_each = toset(["GITHUB_CLIENT_SECRET", "FACEBOOK_CLIENT_SECRET"])

  project   = var.project_id
  secret_id = each.value
  role      = "roles/secretmanager.secretAccessor"
  member    = local.deployer_member
}

# Public access for the Cloud Run service. Managing a service IAM binding
# needs run.services.setIamPolicy, which run.developer does not have, so
# this static binding is managed here instead of in terraform_gcp_app.
resource "google_cloud_run_service_iam_member" "public_access" {
  project  = var.project_id
  location = var.region
  service  = var.app_service_name
  role     = "roles/run.invoker"
  member   = "allUsers"
}

# Custom domain mapping. run.developer has no domainmappings permissions,
# so this static mapping is managed here instead of in terraform_gcp_app.
resource "google_cloud_run_domain_mapping" "river" {
  name     = "river.duxca.com"
  location = var.region

  metadata {
    namespace = var.project_id
  }

  spec {
    route_name       = var.app_service_name
    certificate_mode = "AUTOMATIC"
  }
}
