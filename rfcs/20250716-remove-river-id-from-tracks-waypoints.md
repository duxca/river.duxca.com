# RFC 2025-07-16: RiverTrackとRiverWaypointテーブルからriver_idを除去

**ステータス**: Draft  
**作成者**: Claude Code  
**作成日**: 2025-07-16  
**更新日**: 2025-07-16  

## 概要

このRFCは、`river_tracks`と`river_waypoints`テーブルから`river_id`カラムを除去し、トラックとウェイポイントが特定の川に紐付けられることなく独立して存在できる、より柔軟なデータモデルを提案します。また、必要に応じて多対多の関係を維持するためのジャンクションテーブルを使用する代替アプローチも提案します。

## 背景

現在、`RiverTrack`と`RiverWaypoint`エンティティは、`river_id`外部キーを通じて特定の川に密結合されています。これにより以下の制限が発生しています：

1. **密結合**: トラックとウェイポイントが川から独立して存在できない
2. **柔軟性の欠如**: 一つのトラックやウェイポイントが複数の川に関連付けられない
3. **データベース制約**: 川の削除時にCASCADE DELETEにより関連するトラックとウェイポイントがすべて削除される
4. **概念的なミスアライメント**: トラックとウェイポイントは地理的/航行的データであり、単一の川の文脈を超えて関連性を持つ可能性がある

## 詳細設計

### アプローチ1: 完全なriver_id除去（推奨）

#### データベーススキーマ変更

**変更前（現在の状態）**
```sql
CREATE TABLE river_tracks (
  river_track_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  river_id INTEGER NOT NULL,              -- 削除対象
  user_id INTEGER NOT NULL,
  track_name TEXT NOT NULL,
  description TEXT NOT NULL,
  track JSON NOT NULL,
  created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  FOREIGN KEY (river_id) references rivers(river_id) ON DELETE CASCADE,  -- 削除対象
  FOREIGN KEY (user_id) references users(user_id) ON DELETE CASCADE
);

CREATE TABLE river_waypoints (
  river_waypoint_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  river_id INTEGER NOT NULL,              -- 削除対象
  user_id INTEGER NOT NULL,
  waypoint_name TEXT NOT NULL,
  description TEXT NOT NULL,
  waypoint JSON NOT NULL,
  created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  FOREIGN KEY (river_id) references rivers(river_id) ON DELETE CASCADE,  -- 削除対象
  FOREIGN KEY (user_id) references users(user_id) ON DELETE CASCADE
);
```

**変更後（提案される状態）**
```sql
CREATE TABLE river_tracks (
  river_track_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  user_id INTEGER NOT NULL,
  track_name TEXT NOT NULL,
  description TEXT NOT NULL,
  track JSON NOT NULL,
  created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  FOREIGN KEY (user_id) references users(user_id) ON DELETE CASCADE
);

CREATE TABLE river_waypoints (
  river_waypoint_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  user_id INTEGER NOT NULL,
  waypoint_name TEXT NOT NULL,
  description TEXT NOT NULL,
  waypoint JSON NOT NULL,
  created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  FOREIGN KEY (user_id) references users(user_id) ON DELETE CASCADE
);
```

### アプローチ2: ジャンクションテーブル方式

既存のriver_idを除去し、多対多の関係を実現するためのジャンクションテーブルを追加：

```sql
-- 修正された river_tracks テーブル（river_id除去）
CREATE TABLE river_tracks (
  river_track_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  user_id INTEGER NOT NULL,
  track_name TEXT NOT NULL,
  description TEXT NOT NULL,
  track JSON NOT NULL,
  created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  FOREIGN KEY (user_id) references users(user_id) ON DELETE CASCADE
);

-- 修正された river_waypoints テーブル（river_id除去）
CREATE TABLE river_waypoints (
  river_waypoint_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  user_id INTEGER NOT NULL,
  waypoint_name TEXT NOT NULL,
  description TEXT NOT NULL,
  waypoint JSON NOT NULL,
  created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  FOREIGN KEY (user_id) references users(user_id) ON DELETE CASCADE
);

-- 川とトラックの関連付けテーブル
CREATE TABLE river_track_associations (
  river_id INTEGER NOT NULL,
  river_track_id INTEGER NOT NULL,
  created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  PRIMARY KEY (river_id, river_track_id),
  FOREIGN KEY (river_id) REFERENCES rivers(river_id) ON DELETE CASCADE,
  FOREIGN KEY (river_track_id) REFERENCES river_tracks(river_track_id) ON DELETE CASCADE
);

-- 川とウェイポイントの関連付けテーブル
CREATE TABLE river_waypoint_associations (
  river_id INTEGER NOT NULL,
  river_waypoint_id INTEGER NOT NULL,
  created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  PRIMARY KEY (river_id, river_waypoint_id),
  FOREIGN KEY (river_id) REFERENCES rivers(river_id) ON DELETE CASCADE,
  FOREIGN KEY (river_waypoint_id) REFERENCES river_waypoints(river_waypoint_id) ON DELETE CASCADE
);
```

### データモデル変更

#### Rust構造体（model/src/river.rs）
```rust
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "sql", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct RiverTrack<T = serde_json::Value> {
    pub river_track_id: i64,
    // pub river_id: i64,  // 削除
    pub user_id: i64,
    pub track_name: String,
    pub description: String,
    pub track: T,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "sql", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct RiverWaypoint<T = serde_json::Value> {
    pub river_waypoint_id: i64,
    // pub river_id: i64,  // 削除
    pub user_id: i64,
    pub waypoint_name: String,
    pub description: String,
    pub waypoint: T,
    pub created_at: i64,
    pub updated_at: i64,
}

// ジャンクションテーブル方式用の新しい構造体
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "sql", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct RiverTrackAssociation {
    pub river_id: i64,
    pub river_track_id: i64,
    pub created_at: i64,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "sql", derive(sqlx::FromRow))]
#[serde(rename_all = "camelCase")]
pub struct RiverWaypointAssociation {
    pub river_id: i64,
    pub river_waypoint_id: i64,
    pub created_at: i64,
}
```

## 実装計画

### フェーズ1：データベース移行スクリプト作成
1. [ ] 新しい移行ファイルを作成：`db/migrations/YYYYMMDD_remove_river_id_from_tracks_waypoints.sql`
2. [ ] 既存データのバックアップ戦略を実装
3. [ ] アプローチ1用とアプローチ2用の移行スクリプトを作成
4. [ ] 移行のロールバック手順を文書化

### フェーズ2：データモデル更新
1. [ ] `model/src/river.rs`のRust構造体を更新
   - [ ] `RiverTrack`構造体から`river_id`フィールドを削除
   - [ ] `RiverWaypoint`構造体から`river_id`フィールドを削除
   - [ ] ジャンクション方式の場合、新しい関連付け構造体を追加
2. [ ] すべてのFromトレイト実装を更新
3. [ ] 単体テストを更新

### フェーズ3：データベース層関数更新
1. [ ] `db/src/river_tracks.rs`の更新
   - [ ] `create_river_track()`関数から`river_id`パラメータを削除
   - [ ] `list_river_tracks_all()`関数を`list_river_tracks()`にリネームし、`river_id`パラメータを削除
   - [ ] ジャンクション方式の場合、関連付け管理関数を追加
2. [ ] `db/src/river_waypoints.rs`の更新
   - [ ] `create_river_waypoint()`関数から`river_id`パラメータを削除
   - [ ] `list_river_waypoints_all()`関数を`list_river_waypoints()`にリネームし、`river_id`パラメータを削除
   - [ ] ジャンクション方式の場合、関連付け管理関数を追加
3. [ ] すべてのSQLクエリを更新
4. [ ] データベース層のテストを更新

### フェーズ4：サービス層とAPI更新
1. [ ] サービス層関数を更新してriver_id依存関係を除去
2. [ ] APIエンドポイントを更新
3. [ ] 権限チェックロジックを見直し
4. [ ] 川に関連するトラック/ウェイポイントを取得する新しい関数を実装（ジャンクション方式の場合）

### フェーズ5：フロントエンド更新
1. [ ] 川-トラック/ウェイポイント関係に依存するコンポーネントを更新
2. [ ] APIクライアントコードを更新
3. [ ] UIでの関連付け管理機能を実装（ジャンクション方式の場合）

### フェーズ6：統合テストとデプロイ
1. [ ] 統合テストを実行
2. [ ] パフォーマンステストを実行
3. [ ] デプロイメント手順を確認
4. [ ] 本番環境でのデータ移行を実行

## テスト戦略

### 単体テスト
- [ ] 新しいRust構造体のシリアライゼーション/デシリアライゼーションテスト
- [ ] データベース関数の単体テスト
- [ ] 関連付け管理関数のテスト（ジャンクション方式）

### 統合テスト
- [ ] API エンドポイントのテスト
- [ ] データベース移行スクリプトのテスト
- [ ] フロントエンドコンポーネントのテスト

### パフォーマンステスト
- [ ] 大量データでのクエリパフォーマンステスト
- [ ] インデックス効果の検証
- [ ] メモリ使用量の測定

## 展開計画

### 段階的デプロイ
1. **開発環境**: 全変更を適用し、完全なテストを実行
2. **ステージング環境**: 本番データのコピーで移行をテスト
3. **本番環境**: メンテナンス時間中に移行を実行

### ロールバック戦略
- データベース移行の各段階でのスナップショット作成
- コード変更の段階的なフィーチャーフラグ管理
- 移行失敗時の自動ロールバック手順

### モニタリング
- 移行プロセス中のデータ整合性チェック
- パフォーマンス指標の監視
- エラーログの集約と分析

## 検討した代替案

### 1. オプショナルなriver_id
```sql
ALTER TABLE river_tracks ALTER COLUMN river_id DROP NOT NULL;
ALTER TABLE river_waypoints ALTER COLUMN river_id DROP NOT NULL;
```
**メリット**: 既存コードへの影響を最小化  
**デメリット**: nullable フィールドの複雑性、データの一貫性の問題

### 2. タグベースシステム
汎用的なタグシステムを実装し、川をタグとして扱う
**メリット**: 最大の柔軟性  
**デメリット**: 複雑な実装、パフォーマンスの問題

### 3. 段階的移行
river_idを残したまま、ジャンクションテーブルを追加し、段階的に移行
**メリット**: リスクの分散  
**デメリット**: 長期間のデータ重複、実装の複雑性

## リスクと軽減策

### 技術的リスク
- **データ損失**: 
  - 軽減策: 複数のバックアップ、段階的移行、徹底的なテスト
- **パフォーマンス劣化**: 
  - 軽減策: 適切なインデックス設計、パフォーマンステスト
- **後方互換性**: 
  - 軽減策: バージョン管理、段階的API更新

### ビジネスリスク
- **ユーザー体験の変化**: 
  - 軽減策: ユーザーへの事前通知、段階的UI更新
- **データ整理の困難**: 
  - 軽減策: 新しい検索・フィルタリング機能の実装

### 運用リスク
- **移行時間の延長**: 
  - 軽減策: 事前の移行時間測定、並列処理の活用
- **システム停止時間**: 
  - 軽減策: オンライン移行手法、段階的切り替え

## 将来の考慮事項

### 拡張性
- ジャンクションテーブル方式により、将来的な他エンティティとの関連付けが容易
- 検索機能の強化により、地理的検索やカテゴリ分類が可能

### メンテナンス性
- より単純なデータモデルによる保守性の向上
- 独立したエンティティによるテストの簡素化

### パフォーマンス
- 適切なインデックス戦略による検索性能の最適化
- キャッシュ戦略の見直し

### 機能拡張
- トラック/ウェイポイントの共有機能
- 複数の川にまたがるルート作成
- 地理的近接性に基づく自動関連付け

---

このRFCは、データ独立性と柔軟性を向上させながら、段階的で安全な移行を可能にする包括的なアプローチを提供します。ジャンクションテーブル方式により、既存の機能を維持しつつ、将来の拡張性を確保できます。