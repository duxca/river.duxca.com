# 自動デプロイ設定ガイド

## 概要
このガイドでは、GitHub Actions を使用して main ブランチへの PR マージ時に自動的に Google Cloud Run にデプロイする設定方法を説明します。

## 設定済みファイル

### 1. 更新されたワークフローファイル
- `.github-workflows-rust-updated.yml` - main ブランチへのプッシュ時に自動デプロイを実行する更新版
- `.github-workflows-deploy-main.yml` - 別案として、デプロイ専用のワークフロー

### 2. 既存のインフラストラクチャ
- `terraform/main.tf` - Google Cloud Run サービスとGCSバケットの設定
- `terraform/outputs.tf` - デプロイ後のサービスURL出力設定
- `terraform/deploy.bash` - Terraform実行スクリプト

## 手動設定手順

### ステップ 1: ワークフローファイルの更新
既存の `.github/workflows/rust.yml` を `.github-workflows-rust-updated.yml` の内容に置き換えてください。

重要な変更点：
```yaml
# 変更前
if: ${{ github.ref == 'refs/heads/production' || github.base_ref == 'production' }}

# 変更後  
if: ${{ github.ref == 'refs/heads/main' && github.event_name == 'push' }}
```

### ステップ 2: 必要な権限とシークレットの確認
以下のGitHub Secretsが設定されていることを確認：
- Google Cloud Workload Identity Pool設定済み
- サービスアカウント `river-container@duxca-298210.iam.gserviceaccount.com` の権限設定済み

### ステップ 3: デプロイフロー
1. 開発ブランチでコード変更
2. main ブランチに向けてプルリクエスト作成
3. プルリクエストをマージ
4. 自動的にデプロイジョブが実行される：
   - コードフォーマット/リント/テストチェック
   - Dockerイメージビルド&プッシュ
   - Terraformによるインフラストラクチャ更新
   - Cloud Runサービスデプロイ

## セキュリティ考慮事項
- Workload Identity を使用してセキュアな認証
- 最小権限原則でサービスアカウント設定
- シークレット情報はGitHub Secretsで管理

## トラブルシューティング
- デプロイが失敗した場合は GitHub Actions のログを確認
- Terraform のステート管理に注意
- Cloud Run のリソース制限（CPU: 1コア, メモリ: 256Mi）に注意

## 既存環境への影響
- 現在 `production` ブランチでデプロイしている設定を `main` ブランチに変更
- 既存のTerraform設定とGoogle Cloudリソースはそのまま利用