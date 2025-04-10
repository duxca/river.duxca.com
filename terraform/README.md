# Cloud Run Terraformデプロイメント

このディレクトリには、Google Cloud Runにサービスをデプロイするためのコードが含まれています。デプロイはTerraformを使用して行われます：

1. **Terraform**: Cloud Runサービスの設定と特定のGCSバケット（`duxca-litestream-sandbox`）へのアクセス権を付与

## 前提条件

- [Terraform](https://www.terraform.io/downloads.html) v1.0.0以上がインストールされていること
- [Google Cloud SDK](https://cloud.google.com/sdk/docs/install)がインストールされていること
- [Google Cloud SDK](https://cloud.google.com/sdk/docs/install)がインストールされていること
- Google Cloudプロジェクトが作成されていること
- 適切なIAM権限が設定されていること（Cloud Run Admin、Service Account User、Storage Admin）

## 認証設定

Terraformを実行する前に、Google Cloudへの認証を行う必要があります。

```bash
gcloud auth application-default login
```

## 使用方法

### 初期化

```bash
cd terraform
terraform init
```

### 計画の確認

```bash
terraform plan
```

### 適用

```bash
./deploy.bash
```

このスクリプトは以下の処理を行います：
1. Dockerイメージのビルドとプッシュ
2. Terraformを使用してCloud Runサービスの設定とGCSバケットへのアクセス権を設定
3. デプロイされたサービスのURLを表示

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

### 環境変数としてのシークレット

`GOOGLE_APPLICATION_CREDENTIALS`シークレットの内容が、環境変数`KEY_JSON`として直接設定されます。これにより、アプリケーションはファイルを介さずに、環境変数から直接Googleの認証情報を取得できます。

```
KEY_JSON=<シークレットの内容>
```

この方法では、ファイルシステムへのアクセスが不要になり、より簡潔な実装が可能になります。

## 出力値

| 出力名 | 説明 |
|--------|------|
| cloud_run_url | デプロイされたCloud Runサービスの公開URL |
| cloud_run_service_name | デプロイされたCloud Runサービスの名前 |
| cloud_run_service_location | デプロイされたCloud Runサービスのリージョン |