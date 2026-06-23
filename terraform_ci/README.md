# CI Terraform

This stack manages GitHub Actions deployment identity:

- GitHub Actions deployer service account
- Workload Identity Pool and Provider
- GitHub `main` branch impersonation binding
- Project IAM for deploys
- Terraform state bucket access
- `roles/iam.serviceAccountUser` on the Cloud Run runtime service account

## Usage

Create the remote state bucket manually before running this stack. The bucket is not managed by Terraform because this stack also uses it as its backend.

```bash
cd terraform_ci
terraform init
terraform plan -var-file=prod.tfvars
terraform apply -var-file=prod.tfvars
```

The GitHub repository secret `CLOUDFLARE_API_TOKEN` is still managed in GitHub, not Terraform.

The Workload Identity provider only accepts OIDC tokens from
`legokichi/river.duxca.com` or `duxca/river.duxca.com` on `refs/heads/main`.
Both repository claims are allowed during the org transfer window. Pull request
workflows should use `terraform init -backend=false`, `validate`, and `fmt`
only; they must not receive `id-token: write`, GCP credentials, or Cloudflare
credentials.
