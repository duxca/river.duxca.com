# RFC 2025-07-16: PR Failure Fixes

**ステータス**: Draft  
**作成者**: Claude Code  
**作成日**: 2025-07-16  
**更新日**: 2025-07-16  

## 概要

現在のPRの失敗を調査し、修正方法を文書化します。主な問題は以下の通りです：

1. **Terraform Plan認証エラー** (PRs #42, #43) - Google Cloud Workload Identity認証の失敗
2. **E2Eテストインフラの問題** (PR #40) - Playwrightテストの実行時間超過
3. **CI/CDパイプラインの信頼性** - 一般的な改善点

## 背景

GitHub Actionsで3つのPRが失敗しています：
- PR #43: terraform-plan job失敗 (認証エラー)
- PR #42: terraform-plan job失敗 (認証エラー)
- PR #40: e2e-tests job失敗 (10分超過でタイムアウト)

これらの失敗により、新機能の開発とマージが阻害されています。

## 詳細設計

### 1. Terraform認証エラーの修正

**問題**: 
```
{"error":"unauthorized_client","error_description":"The given credential is rejected by the attribute condition."}
```

**原因分析**:
Workload Identity PoolのAttribute Conditionが、GitHub ActionsのコンテキストにマッチしていないためGoogle Cloud認証が失敗している。

**修正方法**:
1. Workload Identity Pool の現在の設定を確認
2. Attribute Conditionを正しいGitHubリポジトリパターンに更新
3. GitHub Actionsの権限設定を確認

### 2. E2Eテストの最適化

**問題**: 
E2Eテストが10分以上実行されてタイムアウトする

**原因分析**:
- テストのタイムアウト設定が適切でない
- サーバーの起動待機処理が不十分
- テスト環境のセットアップに時間がかかりすぎる

**修正方法**:
1. Playwrightの設定を最適化
2. サーバー起動の健全性チェックを追加
3. テストのタイムアウト設定を調整

### 3. CI/CDパイプラインの改善

**問題**:
一般的な信頼性とデバッグの問題

**修正方法**:
1. より詳細なログ出力
2. 条件付きジョブ実行の改善
3. 失敗時の通知機能

## 実装計画

### Phase 1: Terraform認証修正 (優先度: 高)

#### タスク1: Workload Identity Pool設定の確認
```bash
# 現在の設定を確認
gcloud iam workload-identity-pools providers describe github \
  --location=global \
  --workload-identity-pool=githubaction \
  --project=duxca-298210
```

#### タスク2: Attribute Conditionの更新
```bash
# 条件を更新してリポジトリを制限
gcloud iam workload-identity-pools providers update github \
  --location=global \
  --workload-identity-pool=githubaction \
  --project=duxca-298210 \
  --attribute-condition="assertion.repository=='legokichi/river.duxca.com'"
```

#### タスク3: GitHub Actionsワークフローの更新
`.github/workflows/check.yml`に以下を追加：
```yaml
- name: Debug Authentication
  run: |
    echo "Actor: ${{ github.actor }}"
    echo "Repository: ${{ github.repository }}"
    echo "Event: ${{ github.event_name }}"
    echo "PR User: ${{ github.event.pull_request.user.login }}"
```

### Phase 2: E2Eテストの最適化 (優先度: 中)

#### タスク1: Playwrightタイムアウト設定の最適化
`tests/e2e/playwright.config.ts`を更新：
```typescript
export default defineConfig({
  timeout: 30 * 1000, // 30秒/テスト
  globalTimeout: 10 * 60 * 1000, // 10分合計
  expect: {
    timeout: 10 * 1000, // 10秒/アサーション
  },
  use: {
    navigationTimeout: 30 * 1000, // 30秒/ナビゲーション
  },
});
```

#### タスク2: サーバー起動健全性チェック
`tests/e2e/run-tests.sh`を更新：
```bash
#!/bin/bash
set -e

wait_for_server() {
    local max_attempts=30
    local attempt=1
    
    while [ $attempt -le $max_attempts ]; do
        if curl -s http://localhost:8080/health >/dev/null 2>&1; then
            echo "Server is ready"
            return 0
        fi
        echo "Waiting for server... ($attempt/$max_attempts)"
        sleep 2
        ((attempt++))
    done
    
    echo "Server failed to start"
    return 1
}

# サーバーをバックグラウンドで起動
cargo run --features=test-mode &
SERVER_PID=$!

# サーバーの準備を待機
wait_for_server

# テストを実行
npm test

# クリーンアップ
kill $SERVER_PID
```

#### タスク3: E2Eテストワークフローの更新
`.github/workflows/e2e-tests.yml`を更新：
```yaml
- name: Run E2E Tests
  run: |
    cd tests/e2e
    npm test
  timeout-minutes: 15
  continue-on-error: false
```

### Phase 3: CI/CDパイプラインの改善 (優先度: 低)

#### タスク1: 条件付きジョブ実行の改善
```yaml
terraform-plan:
  if: |
    github.event.pull_request.user.login == 'legokichi' &&
    github.event_name == 'pull_request'
```

#### タスク2: 失敗時の通知機能
```yaml
- name: Notify on Failure
  if: failure()
  run: |
    echo "::warning::Job failed. Check logs for details."
```

## テスト戦略

### 1. 認証修正のテスト
- 簡単なPRでterraform-planジョブをテスト
- 認証フローが正常に動作することを確認
- 既存の機能に影響がないことを確認

### 2. E2Eテストの検証
- ローカル環境でテストを実行
- CI/CD環境でのテスト実行を確認
- 15分以内での完了を確認

### 3. 回帰テスト
- 既存のCI/CDパイプラインが正常動作することを確認
- 全てのPRでチェックが通ることを確認

## 展開計画

### フェーズ1: 緊急修正 (即時)
1. Workload Identity Pool設定の更新
2. GitHub Actionsワークフローのデバッグ情報追加
3. PRs #42, #43の認証問題修正

### フェーズ2: E2Eテスト修正 (1-2日)
1. Playwrightの設定最適化
2. サーバー起動プロセスの改善
3. PR #40のマージ準備

### フェーズ3: 継続的改善 (1週間)
1. CI/CDパイプラインの全般的な改善
2. 監視とログ機能の追加
3. 文書化の更新

## 検討した代替案

### 1. Terraform認証について
- **代替案1**: 新しいサービスアカウントを作成 → 複雑すぎる
- **代替案2**: 認証を無効化 → セキュリティリスクが高い
- **選択**: 既存のWorkload Identity設定を修正 → 最小限の変更で対応

### 2. E2Eテストについて
- **代替案1**: 別のテストフレームワークを使用 → 大幅な変更が必要
- **代替案2**: テストを無効化 → 品質保証が不十分
- **選択**: Playwrightの設定を最適化 → 既存の投資を活用

## リスクと軽減策

### リスク1: 認証修正による他の機能への影響
- **軽減策**: 段階的な変更とテスト
- **ロールバック**: 元の設定に復元可能

### リスク2: E2Eテストの不安定性
- **軽減策**: 複数回のテスト実行での確認
- **ロールバック**: テストを一時的に無効化

### リスク3: CI/CD変更による開発プロセスへの影響
- **軽減策**: 小さな変更を段階的に実施
- **ロールバック**: Git履歴からの復元

## 将来の考慮事項

### 短期 (1ヶ月)
- CI/CDパイプラインの監視とメトリクス収集
- E2Eテストの安定性向上
- 認証設定の文書化

### 中期 (3ヶ月)
- CI/CDパイプラインの全面的な見直し
- テストの並列実行
- 自動デプロイメントの改善

### 長期 (6ヶ月)
- 複数環境でのテスト自動化
- パフォーマンステストの統合
- セキュリティテストの自動化

## 受け入れ基準

### 必須条件
- [ ] PRs #42, #43のterraform-planジョブが成功する
- [ ] PR #40のe2e-testsジョブが15分以内に完了する
- [ ] 既存のCI/CDパイプラインが正常動作する
- [ ] 新しいPRでもCI/CDチェックが通る

### テスト条件
- [ ] 認証フローのテストが成功する
- [ ] E2EテストがローカルとCI/CDで動作する
- [ ] 回帰テストが全て通る

### 文書化条件
- [ ] 修正内容がCLAUDE.mdに反映される
- [ ] トラブルシューティングガイドが更新される
- [ ] 関連するREADMEが更新される

---

*このRFCは、現在のPR失敗問題の体系的な解決を目的としています。実装は段階的に行い、各フェーズでテストと検証を実施します。*