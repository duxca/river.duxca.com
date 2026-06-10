#!/bin/bash
set -euxvo pipefail

# Change to project root directory
cd "$(dirname "$0")/.."

docker build . --tag=asia-northeast1-docker.pkg.dev/duxca-298210/cloud-run-source-deploy/litestream-sandbox:latest
docker push asia-northeast1-docker.pkg.dev/duxca-298210/cloud-run-source-deploy/litestream-sandbox:latest
gcloud run deploy litestream-sandbox\
  --image=asia-northeast1-docker.pkg.dev/duxca-298210/cloud-run-source-deploy/litestream-sandbox:latest \
  --region=asia-northeast1 \
  --execution-environment=gen1 \
  --cpu=1 \
  --memory=256Mi \
  --timeout=3s \
  --concurrency=128 \
  --max-instances=1 \
  --min-instances=0 \
  --no-cpu-boost \
  --cpu-throttling \
  --service-account river-container@duxca-298210.iam.gserviceaccount.com \
  --update-secrets=FACEBOOK_CLIENT_ID=FACEBOOK_CLIENT_ID:1 \
  --update-secrets=FACEBOOK_CLIENT_SECRET=FACEBOOK_CLIENT_SECRET:1 \
  --update-secrets=GITHUB_CLIENT_ID=GITHUB_CLIENT_ID:1 \
  --update-secrets=GITHUB_CLIENT_SECRET=GITHUB_CLIENT_SECRET:1 \
  --allow-unauthenticated

# ボリュームマウントするとlitestreamがハングする
