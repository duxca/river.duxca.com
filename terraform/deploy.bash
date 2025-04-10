#!/bin/bash
set -euxvo pipefail

# Dockerイメージのビルドとプッシュ
docker build . --tag=asia-northeast1-docker.pkg.dev/duxca-298210/cloud-run-source-deploy/litestream-sandbox:latest
docker push asia-northeast1-docker.pkg.dev/duxca-298210/cloud-run-source-deploy/litestream-sandbox:latest

# Terraformの実行（Cloud RunサービスとGCSバケットへのアクセス権を設定）
cd "$(dirname "$0")"
terraform init
terraform apply -auto-approve

# デプロイ結果の表示
echo "デプロイが完了しました。サービスURLは以下の通りです："
terraform output cloud_run_url