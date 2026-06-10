# Cloud Run Terraformデプロイメント

このディレクトリには、Google Cloud Runにサービスをデプロイするためのコードが含まれています。デプロイはTerraformを使用して行われます：

1. **Terraform**: Cloud Runサービスの設定と特定のGCSバケット（`duxca-litestream-sandbox`）へのアクセス権を付与

## 前提条件

- [Terraform](https://www.terraform.io/downloads.html) v1.0.0以上がインストールされていること
- [Google Cloud SDK](https://cloud.google.com/sdk/docs/install)がインストールされていること
- Google Cloudプロジェクトが作成されていること
- 適切なIAM権限が設定されていること（Cloud Run Admin、Service Account User、Storage Admin）

## 認証設定

Terraformを実行する前に、Google CloudとCloudflareへの認証を行う必要があります。

通常はApplication Default Credentials（ADC）を設定します。

```bash
gcloud auth application-default login
```

ブラウザを開けない環境では、gcloudの現在のactive accountのaccess tokenをTerraformに渡します。

```bash
gcloud auth list
gcloud config list

export GOOGLE_OAUTH_ACCESS_TOKEN="$(gcloud auth print-access-token)"
```

この方法では、tokenの有効期限が切れたら再度 `GOOGLE_OAUTH_ACCESS_TOKEN` を設定し直します。
`terraform init` のGCS backendもこのtokenで認証されます。

Cloudflare DNSもTerraform管理対象なので、Cloudflareをrefresh/applyする場合はAPI tokenも設定します。

```bash
export CLOUDFLARE_API_TOKEN="..."
```

Cloudflareを変更しない一時的な作業でtokenがない場合は `-refresh=false` でGoogle側だけを適用できます。ただし、Cloudflareの実状態を読まずにplan/applyするため、通常運用では `CLOUDFLARE_API_TOKEN` を設定してください。

## 使用方法

### 初期化

```bash
cd terraform
terraform init
```

### 計画の確認

```bash
terraform plan -var="container_image=asia-northeast1-docker.pkg.dev/duxca-298210/cloud-run-source-deploy/litestream-sandbox@sha256:<digest>"
```

### 適用

```bash
terraform apply -var="container_image=asia-northeast1-docker.pkg.dev/duxca-298210/cloud-run-source-deploy/litestream-sandbox@sha256:<digest>"
```

CIでは Docker イメージを push したあと、push 済みイメージの digest を取得し、`container_image` に `image@sha256:...` を渡して Terraform apply します。これにより Cloud Run revision は `:latest` ではなく実際に push された immutable image に固定されます。

ローカルでDocker imageをbuild/pushしてからapplyする例です。

```bash
IMAGE_REPOSITORY="asia-northeast1-docker.pkg.dev/duxca-298210/cloud-run-source-deploy/litestream-sandbox"

gcloud auth configure-docker asia-northeast1-docker.pkg.dev
docker buildx build . \
  --tag="${IMAGE_REPOSITORY}:latest" \
  --push \
  --metadata-file=/tmp/river-image-metadata.json

IMAGE_DIGEST="$(jq -r '."containerimage.digest"' /tmp/river-image-metadata.json)"
CONTAINER_IMAGE="${IMAGE_REPOSITORY}@${IMAGE_DIGEST}"

cd terraform
terraform init
terraform plan -var="container_image=${CONTAINER_IMAGE}"
terraform apply -var="container_image=${CONTAINER_IMAGE}"
```

### 破棄

```bash
terraform destroy  # Cloud RunサービスとGCSバケットへのアクセス権を削除
```

## カスタマイズ

`variables.tf`ファイルには、デフォルト値が設定されていますが、以下の方法でカスタマイズできます：

1. コマンドライン引数で指定：
   ```bash
   terraform apply -var="project_id=your-project-id" -var="region=us-central1"
   ```

2. `terraform.tfvars`ファイルを作成：
   ```hcl
   project_id = "your-project-id"
   region     = "us-central1"
   ```

3. 環境変数で指定：
   ```bash
   export TF_VAR_project_id=your-project-id
   export TF_VAR_region=us-central1
   terraform apply
   ```

## 変数

| 変数名 | 説明 | デフォルト値 |
|--------|------|-------------|
| project_id | Google CloudプロジェクトのID | duxca-298210 |
| region | デプロイするリージョン | asia-northeast1 |
| service_name | Cloud Runサービスの名前 | litestream-sandbox |
| container_image | デプロイするコンテナイメージのURL | asia-northeast1-docker.pkg.dev/duxca-298210/cloud-run-source-deploy/litestream-sandbox:latest |
| service_account_email | サービスで使用するサービスアカウントのメールアドレス | river-container@duxca-298210.iam.gserviceaccount.com |

## 付与されるIAMロール

## IAMロールとシークレットのマウント

### IAMロール

サービスアカウントには以下のIAMロールが付与されます：

1. `roles/run.invoker` - Cloud Runサービスを呼び出す権限（allUsersに付与）

また、特定のGCSバケット（`duxca-litestream-sandbox`）に対して以下の権限が付与されます：

1. `roles/storage.objectAdmin` - バケット内のオブジェクトの作成、読み取り、更新、削除などの権限
2. `roles/storage.legacyBucketReader` - バケットのメタデータの読み取り権限
3. `roles/storage.legacyBucketWriter` - バケットへのオブジェクトの書き込み権限

### 認証とシークレット

OAuth の client ID / secret は Secret Manager から Cloud Run の環境変数として渡します。
GCS へのアクセスは JSON key をコンテナへ渡さず、Cloud Run の service account と metadata server の ADC を使います。

## 出力値

| 出力名 | 説明 |
|--------|------|
| cloud_run_url | デプロイされたCloud Runサービスの公開URL |
| cloud_run_service_name | デプロイされたCloud Runサービスの名前 |
| cloud_run_service_location | デプロイされたCloud Runサービスのリージョン |
