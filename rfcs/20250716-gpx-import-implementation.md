# GPXインポート機能実装RFC

**作成日:** 2025-07-16 19:06:00 UTC  
**ステータス:** ドラフト  
**作成者:** 既存コードベースと要件の分析

## 概要

このRFCは、river.duxca.comアプリケーションにGPXファイルインポート機能を追加するための実装計画を説明します。この機能により、ユーザーは河川のトラックデータとウェイポイントデータを含むGPXファイルを直接システムにインポートできるようになります。

## 現状分析

### データベーススキーマ

`db/migrations/20240817131736_rivers.sql`の既存データベーススキーマでは以下を提供しています：

- **riversテーブル**: 代表ウェイポイントを含む河川の基本情報
- **river_tracksテーブル**: `[[lat, long], ...]`形式のJSON配列としてトラックデータを格納
- **river_waypointsテーブル**: `[lat, long]`座標を持つ個別のウェイポイント

### API設計

現在のAPIは以下のタグ付きユニオンパターンに従っています：
- `model/src/api/`内のリクエスト/レスポンスタイプ
- `service/src/`内の一元化されたハンドラー
- `Request::check_permission()`による権限チェック
- `POST /api`経由のJSONベース通信

### GPXファイル分析

`gpx/`ディレクトリ内で2つの異なるタイプのGPXファイルを確認しました：

#### タイプ1: Geographicaアプリファイル（10個）
- **作成者:** Geographicaアプリ
- **形式:** トラックデータ（`<trkpt>`要素）
- **内容:** タイムスタンプ、標高、速度を含む詳細なGPS追跡データ
- **例:** `20250717_天塩中川.gpx`、`20250717_天塩佐久.gpx`など
- **サイズ:** 117-282行のXML

#### タイプ2: NPS（Navigation Planning System）ファイル（5個）
- **作成者:** new pec smartアプリ（`jp.mappleon.nps`）
- **形式:** ルートデータ（`<rtept>`要素）
- **内容:** 海事ナビゲーションルート計画
- **例:** `nps_plan_1_20250717_014758.gpx`、`nps_plan_2_20250717_014758.gpx`
- **サイズ:** 1,385-1,509バイト（単行形式）

## 実装計画

### 1. 依存関係

`server/Cargo.toml`に追加：
```toml
gpx = "0.10"
geo-types = "0.7"
```

**根拠:** georustの`gpx`クレートは、GPX 1.0と1.1の両方の形式をサポートする、最も成熟した包括的なRust用GPX解析ライブラリです。

### 2. API定義

`model/src/api/import_gpx.rs`を作成：
```rust
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub river_id: Option<i64>,  // オプション：未指定の場合は新しい河川を作成
    pub file_content: String,   // Base64エンコードされたGPXファイル内容
    pub import_options: ImportOptions,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ImportOptions {
    pub import_tracks: bool,      // トラックをインポートするか
    pub import_routes: bool,      // ルートをインポートするか
    pub import_waypoints: bool,   // ウェイポイントをインポートするか
    pub create_river_if_missing: bool,  // 河川が存在しない場合作成するか
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub river_id: i64,
    pub imported_tracks: Vec<i64>,
    pub imported_waypoints: Vec<i64>,
    pub summary: ImportSummary,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ImportSummary {
    pub tracks_imported: u32,
    pub waypoints_imported: u32,
    pub routes_imported: u32,
}
```

### 3. データマッピング戦略

#### トラックデータマッピング
- **GPXトラック（`<trk>` → `<trkpt>`）**: `river_tracks`テーブルにマッピング
- **GPXルート（`<rte>` → `<rtept>`）**: `river_tracks`テーブルにマッピング
- **座標形式**: `[[lat, long], ...]`として`Vec<(f64, f64)>`に変換

#### ウェイポイントデータマッピング
- **GPXウェイポイント（`<wpt>`）**: `river_waypoints`テーブルにマッピング
- **座標形式**: `[lat, long]`として`(f64, f64)`に変換

#### 河川作成ロジック
- `river_id`が指定されている場合：既存の河川を使用
- `river_id`が未指定で`create_river_if_missing=true`の場合：新しい河川を作成
- 河川情報にGPXメタデータ（名前、説明）を使用
- トラック/ルートの重心から代表ウェイポイントを計算

### 4. サービス実装

`service/src/import_gpx.rs`を作成：
```rust
pub async fn import_gpx(
    db: &sqlx::SqlitePool,
    user: &model::user::User,
    request: model::api::import_gpx::Request,
) -> Result<model::api::import_gpx::Response, anyhow::Error> {
    // 1. `gpx`クレートを使用してGPXファイルを解析
    // 2. トラック、ルート、ウェイポイントを抽出
    // 3. 必要に応じて河川を作成
    // 4. river_tracksにトラックデータを挿入
    // 5. river_waypointsにウェイポイントデータを挿入
    // 6. サマリーを返す
}
```

### 5. 権限モデル

インポート権限を以下に付与：
- **管理者ユーザー（role=0）**: 完全なインポートアクセス
- **一般ユーザー（role=1）**: 自分の河川のみにインポート可能
- **検証**: ユーザーが対象河川を変更できることを確認

### 6. エラーハンドリング

一般的な失敗シナリオを処理：
- 無効なGPX形式
- 必須フィールドの欠如
- 権限拒否
- データベース制約違反
- 大容量ファイル処理制限

### 7. ファイルアップロード機構

検討された2つのアプローチ：

#### オプションA: マルチパートアップロード
- ファイルアップロードにaxum multipartを使用
- Base64エンコードなしでの直接ファイル処理
- 大容量ファイルでより効率的

#### オプションB: JSON Base64（推奨）
- 既存のAPIパターンとの一貫性
- よりシンプルなクライアント実装
- より良いエラーハンドリング統合

## 技術的考慮事項

### パフォーマンス
- **メモリ使用量**: 大容量GPXファイルでのストリーミング解析
- **データベーストランザクション**: ロールバック機能付きのアトミックインポート
- **検証**: 部分的インポートを防ぐための事前インポート検証

### データ整合性
- **座標検証**: 有効な緯度/経度範囲の確認
- **重複防止**: 既存のトラック/ウェイポイントのチェック
- **参照整合性**: インポート前にriver_idの存在確認

### 拡張性
- **プラグイン設計**: 追加GPS形式のサポート
- **メタデータ保存**: 元のGPXメタデータの保存
- **バッチ処理**: 複数ファイルインポートのサポート

## 実装フェーズ

### フェーズ1: コア実装
1. GPX解析依存関係の追加
2. 基本APIエンドポイントの実装
3. サービス層ロジックの作成
4. データベース操作の追加

### フェーズ2: 拡張機能
1. 包括的エラーハンドリングの追加
2. 権限検証の実装
3. インポートオプションサポートの追加
4. 単体テストの作成

### フェーズ3: 統合
1. フロントエンド統合
2. ファイルアップロードUI
3. インポート進捗追跡
4. ユーザードキュメント

## 検討された代替アプローチ

### 1. ファイルストレージアプローチ
GPXファイルをGCSに保存し、データベースで参照
- **利点**: 元データの保存、大容量ファイルサポート
- **欠点**: 複雑性の増加、ストレージコスト、処理オーバーヘッド

### 2. 専用GPXテーブル
GPXメタデータ用の専用テーブルを作成
- **利点**: クリーンな分離、クエリ性能向上
- **欠点**: スキーマの複雑化、データ重複

### 3. バックグラウンド処理
ジョブキューを使用した非同期GPX処理
- **利点**: 大容量ファイルでのUX向上、スケーラビリティ
- **欠点**: 実装の複雑化、状態管理

## 結論

提案された実装は、既存のコードベース設計との一貫性を保ちながら、GPXインポート機能の堅牢な基盤を提供します。成熟した`gpx`クレートの使用により、実際に遭遇する多様なGPX形式の信頼性のある解析が保証されます。

段階的アプローチにより、反復的な開発とテストが可能になり、コア機能は即座に価値を提供しながら、高度な機能は段階的に追加できます。