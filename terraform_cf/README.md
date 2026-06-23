# Cloudflare Terraform

This directory manages Cloudflare DNS records for `duxca.com`.

## Authentication

For local applies, use the OAuth token managed by the `cf` CLI:

```bash
cf auth whoami >/dev/null

CLOUDFLARE_API_TOKEN="$(awk -F ' = ' '/^access_token = / {gsub(/"/, "", $2); print $2}' ~/.cf/config.toml)" \
  terraform plan -var-file=prod.tfvars
```

The Cloudflare Terraform provider does not read `cf` CLI config directly, so the access token is passed to the Terraform process as `CLOUDFLARE_API_TOKEN`.

## Backend

The remote state uses the shared Terraform state bucket with a Cloudflare-only prefix:

```bash
terraform init -reconfigure \
  -backend-config="bucket=river-duxca-prod-terraform-state" \
  -backend-config="prefix=river.duxca.com/cf"
```

## Usage

```bash
cd terraform_cf
CLOUDFLARE_API_TOKEN="$(awk -F ' = ' '/^access_token = / {gsub(/"/, "", $2); print $2}' ~/.cf/config.toml)" \
  terraform plan -var-file=prod.tfvars
CLOUDFLARE_API_TOKEN="$(awk -F ' = ' '/^access_token = / {gsub(/"/, "", $2); print $2}' ~/.cf/config.toml)" \
  terraform apply -var-file=prod.tfvars
```

Apply this after the GCP Cloud Run domain mapping exists. The DNS target is `ghs.googlehosted.com`.

See the root `README.md` for the full deployment flow.
