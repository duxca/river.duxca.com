#!/bin/bash
set -euxo pipefail

NAME=asia-northeast1-docker.pkg.dev/duxca-298210/cloud-run-source-deploy/litestream-sandbox:latest
#docker build . --tag=$NAME
#docker push $NAME
gcloud run deploy litestream-sandbox\
  --image=$NAME \
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
  --update-env-vars=GOOGLE_APPLICATION_CREDENTIALS=/etc/key.json \
#  --update-secrets=/etc/key.json=GOOGLE_APPLICATION_CREDENTIALS:1 \
#  --update-secrets=FACEBOOK_CLIENT_ID=FACEBOOK_CLIENT_ID:1 \
#  --update-secrets=FACEBOOK_CLIENT_SECRET=FACEBOOK_CLIENT_SECRET:1 \
#  --update-secrets=GITLAB_CLIENT_ID=GITLAB_CLIENT_ID:1 \
#  --update-secrets=GITLAB_CLIENT_SECRET=GITLAB_CLIENT_SECRET:1 \
#  --update-secrets=TWITTER_CLIENT_ID=TWITTER_CLIENT_ID:1 \
#  --update-secrets=TWITTER_CLIENT_SECRET=TWITTER_CLIENT_SECRET:1
#  --use-http2 \
  --allow-unauthenticated
