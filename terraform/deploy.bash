#!/bin/bash
set -euxvo pipefail

# Dockerイメージのビルドとプッシュ
docker build . --tag=asia-northeast1-docker.pkg.dev/duxca-298210/cloud-run-source-deploy/litestream-sandbox:latest
docker push asia-northeast1-docker.pkg.dev/duxca-298210/cloud-run-source-deploy/litestream-sandbox:latest

# Terraformの実行（GCSバケットへのアクセス権を設定）
cd "$(dirname "$0")"
terraform init
terraform apply -auto-approve

# gcloudコマンドでCloud Runサービスをデプロイ（シークレットをファイルとしてマウント）
gcloud run deploy litestream-sandbox \
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
  --update-secrets=TWITTER_CLIENT_ID=TWITTER_CLIENT_ID:1 \
  --update-secrets=TWITTER_CLIENT_SECRET=TWITTER_CLIENT_SECRET:1 \
  --update-secrets=KEY_JSON=GOOGLE_APPLICATION_CREDENTIALS:3 \
  --allow-unauthenticated

# デプロイ結果の表示
echo "デプロイが完了しました。サービスURLは以下の通りです："
gcloud run services describe litestream-sandbox --region=asia-northeast1 --format='value(status.url)'