#!/bin/bash
set -e

echo "🚀 Migrating Terraform state to Google Cloud Storage"

# Step 1: Create the state bucket using bootstrap
echo "📦 Step 1: Creating GCS bucket for Terraform state..."
cd bootstrap
terraform init
terraform plan -out=bootstrap.tfplan
echo "Creating state bucket..."
terraform apply bootstrap.tfplan
cd ..

# Step 2: Initialize main Terraform with GCS backend
echo "🔄 Step 2: Migrating existing state to GCS..."
echo "Terraform will prompt you to migrate the existing state."
echo "Answer 'yes' when prompted."
terraform init

# Step 3: Verify the migration
echo "✅ Step 3: Verifying state migration..."
terraform state list

echo "🎉 Migration complete!"
echo "State is now stored in GCS bucket: duxca-terraform-state"
echo "You can safely delete the local terraform.tfstate files."