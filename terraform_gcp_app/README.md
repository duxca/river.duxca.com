# GCP App Terraform

This stack manages the Cloud Run application resources:

- Cloud Run service
- Public invoker IAM binding
- Cloud Run domain mapping for `river.duxca.com`

It reads bucket and service account outputs from `../terraform_gcp_storage` via `terraform_remote_state`.

## Backend

```bash
terraform init -reconfigure \
  -backend-config="bucket=river-duxca-prod-terraform-state" \
  -backend-config="prefix=river.duxca.com/gcp_app"
```

## Usage

Apply `../terraform_gcp_storage` first, then push a container image and add secret versions.

```bash
cd terraform_gcp_app
terraform plan \
  -var-file=prod.tfvars \
  -var="container_image=asia-northeast1-docker.pkg.dev/river-duxca-prod/cloud-run-source-deploy/litestream-sandbox@sha256:<digest>"

terraform apply \
  -var-file=prod.tfvars \
  -var="container_image=asia-northeast1-docker.pkg.dev/river-duxca-prod/cloud-run-source-deploy/litestream-sandbox@sha256:<digest>"
```

Apply `../terraform_cf` after this stack creates the Cloud Run domain mapping. See the root `README.md` for the full deployment flow.
