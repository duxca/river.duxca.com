# RFC: RiverTrackとRiverWaypointテーブルからriver_idを除去

## 概要

このRFCは、`river_tracks`と`river_waypoints`テーブルから`river_id`カラムを除去し、トラックとウェイポイントが特定の川に紐付けられることなく独立して存在できる、より柔軟なデータモデルを提案します。

## 動機

現在、`RiverTrack`と`RiverWaypoint`エンティティは、`river_id`外部キーを通じて特定の川に密結合されています。これにより以下の制限が発生しています：

1. **密結合**: トラックとウェイポイントが川から独立して存在できない
2. **柔軟性の欠如**: 一つのトラックやウェイポイントが複数の川に関連付けられない
3. **データベース制約**: 川の削除時にCASCADE DELETEにより関連するトラックとウェイポイントがすべて削除される
4. **概念的なミスアライメント**: トラックとウェイポイントは地理的/航行的データであり、単一の川の文脈を超えて関連性を持つ可能性がある

## 詳細設計

### データベーススキーマ変更

#### 変更前（現在の状態）
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

#### 変更後（提案される状態）
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
```

### API変更

#### データベース層（db/src/）

**db/src/river_tracks.rs**
- `create_river_track()`関数から`river_id`パラメータを削除
- `list_river_tracks_all()`関数から`river_id`パラメータを削除（`list_river_tracks()`にリネーム）
- すべてのSQLクエリから`river_id`参照を削除
- テストから`river_id`依存関係を削除

**db/src/river_waypoints.rs**
- `create_river_waypoint()`関数から`river_id`パラメータを削除
- `list_river_waypoints_all()`関数から`river_id`パラメータを削除（`list_river_waypoints()`にリネーム）
- すべてのSQLクエリから`river_id`参照を削除
- テストから`river_id`依存関係を削除

#### サービス層変更
- トラック/ウェイポイント操作に`river_id`を必要としないようにサービス関数を更新
- 川-トラック/ウェイポイント関係に依存するビジネスロジックを修正
- 必要に応じて権限チェックを更新

## 移行戦略

### フェーズ1：データベース移行
1. 新しい移行ファイルを作成：`db/migrations/YYYYMMDD_remove_river_id_from_tracks_waypoints.sql`
2. 外部キー制約を削除
3. 両テーブルから`river_id`カラムを削除
4. `river_id`を参照するインデックスを更新

### フェーズ2：コード更新
1. `model/src/river.rs`のRust構造体を更新
2. `db/src/river_tracks.rs`と`db/src/river_waypoints.rs`のデータベース関数を更新
3. サービス層関数を更新
4. APIエンドポイントを更新
5. フロントエンドコンポーネントを更新

### フェーズ3：テスト
1. 単体テストを更新
2. 統合テストを更新
3. river_id制約なしでの全機能動作を確認

## 影響分析

### 破壊的変更
- **API**: 以前に`river_id`パラメータを受け取っていた関数の更新が必要
- **データベース**: `river_id`でフィルタリングする既存のクエリのリファクタリングが必要
- **フロントエンド**: 川-トラック/ウェイポイント関係に依存するコンポーネントの更新が必要

### 現在の使用状況分析
コード分析に基づき、以下の関数が影響を受けます：

**db/src/river_tracks.rs:**
- `create_river_track()` - river_idパラメータを削除
- `list_river_tracks_all()` - river_idパラメータを削除、`list_river_tracks()`にリネーム

**db/src/river_waypoints.rs:**
- `create_river_waypoint()` - river_idパラメータを削除
- `list_river_waypoints_all()` - river_idパラメータを削除、`list_river_waypoints()`にリネーム

### データ整合性の考慮事項
- **孤立データ**: 移行後、トラックとウェイポイントは川が削除されても自動的に削除されない
- **参照整合性**: 必要に応じて適切なクリーンアップメカニズムを確保する必要がある
- **クエリパフォーマンス**: 一般的なクエリパターンに対して新しいインデックスが必要な場合がある

## 代替アプローチ

### 1. 多対多関係
トラック/ウェイポイントが複数の川に関連付けられるようにジャンクションテーブルを作成：
```sql
CREATE TABLE river_track_associations (
  river_id INTEGER NOT NULL,
  river_track_id INTEGER NOT NULL,
  PRIMARY KEY (river_id, river_track_id),
  FOREIGN KEY (river_id) REFERENCES rivers(river_id) ON DELETE CASCADE,
  FOREIGN KEY (river_track_id) REFERENCES river_tracks(river_track_id) ON DELETE CASCADE
);
```

### 2. オプショナルな川関連付け
完全に削除する代わりに`river_id`をnullable にする：
```sql
ALTER TABLE river_tracks ALTER COLUMN river_id DROP NOT NULL;
```

### 3. タグシステム
トラック/ウェイポイントを川やその他のエンティティに関連付けるタグシステムを実装。

## リスクと考慮事項

### 技術的リスク
- **データ損失**: 移行が失敗した場合、既存の川-トラック/ウェイポイント関係が失われる可能性がある
- **パフォーマンス**: river_idフィルタリングがないと、一部のクエリが遅くなる可能性がある
- **後方互換性**: 既存のAPIクライアントが動作しなくなる可能性がある

### ビジネスリスク
- **ユーザー体験**: ユーザーはトラック/ウェイポイントが川別に整理されることを期待する可能性がある
- **データ整理**: 川の関連付けがないと、データの整理が困難になる可能性がある

### 軽減策
1. **包括的テスト**: デプロイ前の広範囲なテスト
2. **ロールバック計画**: 必要に応じてデータベース変更を元に戻す機能
3. **フィーチャーフラグ**: フィーチャーフラグを使用した段階的な変更の展開
4. **ユーザーコミュニケーション**: データ整理の変更についてユーザーに通知

## 実装タイムライン

1. **第1週**: 移行スクリプトの作成とデータモデルの更新
2. **第2週**: データベース層関数とテストの更新
3. **第3週**: サービス層とAPIエンドポイントの更新
4. **第4週**: フロントエンドコンポーネントと統合テストの更新
5. **第5週**: テストとデプロイ

## 結論

トラックとウェイポイントテーブルから`river_id`を除去することで、より柔軟で独立したデータモデルが構築されます。この変更には大規模なリファクタリングが必要ですが、システムがより複雑なユースケースを処理し、将来的により良いデータ整理機能を提供する能力を向上させます。

提案されたアプローチは柔軟性と実装の複雑さのバランスを取り、段階的な移行戦略はリスクを最小化しながらシステムの安定性を確保します。