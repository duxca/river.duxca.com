# Terraform Bootstrap

This directory contains the bootstrap configuration to create the GCS bucket for Terraform state storage.

## Usage

1. **First-time setup (run this once):**
   ```bash
   cd terraform_gcp_storage/bootstrap
   terraform init
   terraform plan -var-file=prod.tfvars
   terraform apply -var-file=prod.tfvars
   ```

   Use `old.tfvars` only when operating on the existing project.

2. **After the state bucket is created, initialize the storage and app states:**
   ```bash
   cd ../
   terraform init -reconfigure \
     -backend-config="bucket=river-duxca-prod-terraform-state" \
     -backend-config="prefix=river.duxca.com/gcp_storage"

   cd ../terraform_gcp_app
   terraform init -reconfigure \
     -backend-config="bucket=river-duxca-prod-terraform-state" \
     -backend-config="prefix=river.duxca.com/gcp_app"
   ```

## Important Notes

- This bootstrap should only be run once to create the state bucket
- The state bucket has versioning enabled and lifecycle management
- The bucket has `prevent_destroy = true` to avoid accidental deletion
