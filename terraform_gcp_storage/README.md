# GCP Storage Terraform

This stack manages the base Google Cloud resources needed before the app can be deployed:

- Required project APIs
- Artifact Registry repository
- Litestream GCS bucket
- Cloud Run service account
- Secret Manager secret containers
- IAM bindings for the app service account

## Backend

```bash
terraform init -reconfigure \
  -backend-config="bucket=river-duxca-prod-terraform-state" \
  -backend-config="prefix=river.duxca.com/gcp_storage"
```

## Usage

```bash
cd terraform_gcp_storage
terraform plan -var-file=prod.tfvars
terraform apply -var-file=prod.tfvars
```

After this stack is applied:

1. Add Secret Manager secret versions.
2. Push the container image to Artifact Registry.
3. Apply `../terraform_gcp_app`.

See the root `README.md` for the full deployment flow.
