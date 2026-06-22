# Terraform Bootstrap

This stack manages the GCS bucket used by the other Terraform stacks as their remote state backend.

## Usage

Run this stack before any stack that uses the GCS backend.

```bash
cd terraform_bootstrap
terraform init
terraform plan -var-file=prod.tfvars
terraform apply -var-file=prod.tfvars
```

Use `old.tfvars` only when operating on the old project.

## Notes

- This stack uses local Terraform state by design. It cannot use the GCS bucket before the bucket exists.
- The bucket has object versioning, uniform bucket-level access, public access prevention, and lifecycle cleanup for old object versions.
- `prevent_destroy = true` protects the state bucket from accidental deletion.
- The backend prefixes used by the other stacks are plain object path prefixes, not separate GCS resources.
