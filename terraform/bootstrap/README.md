# Terraform Bootstrap

This directory contains the bootstrap configuration to create the GCS bucket for Terraform state storage.

## Usage

1. **First-time setup (run this once):**
   ```bash
   cd terraform/bootstrap
   terraform init
   terraform plan
   terraform apply
   ```

2. **After the state bucket is created, migrate the main Terraform state:**
   ```bash
   cd ../
   terraform init  # This will prompt to migrate existing state to GCS
   ```

## Important Notes

- This bootstrap should only be run once to create the state bucket
- The state bucket has versioning enabled and lifecycle management
- The bucket has `prevent_destroy = true` to avoid accidental deletion