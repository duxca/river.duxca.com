# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

# River.duxca.com プロジェクト概要

このソフトウェアは川の地図情報を処理するWebアプリケーションです。カヌー、カヤック、SUPなどの川遊びに役立つ情報を地図上に表示し、ユーザーが情報を追加・共有できるプラットフォームを提供します。

## プロジェクト構成

### サーバーサイド (server/)
- Rustの**axum**フレームワークを使用したWebサーバー
- OAuth認証（GitHub、Facebook、Twitter）によるユーザー認証
- RESTful APIエンドポイントの提供
- 管理者向け管理画面の提供

### フロントエンド (browser/)
- Rustの**Yew**フレームワークを使用したSPA（Single Page Application）
- **leaflet.js**のRustラッパーライブラリを使用した地図表示
- 地理院タイル、OpenStreetMapなど複数の地図レイヤー対応
- 川の情報、ウェイポイント、トラック（ルート）の表示・編集機能

### データベース (db/)
- **SQLite3**データベースを使用したデータ永続化
- **sqlx**を使用したRustからのデータベースアクセス
- **Litestream**を使用したGCSへのデータベースレプリケーション
- マイグレーションスクリプトによるスキーマ管理（db/migrations/）
- マイグレーションの実行方法：`cd db && sqlx migrate run; cd ..`
- データベースのリセット方法：`cd db && sqlx database reset; cd ..`

### ドメインモデル (model/)
- サーバー、ブラウザ、データベース間で共有される型定義
- 川（River）、ウェイポイント（RiverWaypoint）、トラック（RiverTrack）などの構造体
- APIリクエスト/レスポンスの型定義
- ユーザー認証関連の型定義

### サービス層 (service/)
- Webサーバーが提供するAPIの具体的な処理実装
- ユーザー権限チェック
- データベースとの連携処理
- ビジネスロジックの実装

## 主要機能

1. **地図表示**：地理院タイル、OpenStreetMap、航空写真などの複数レイヤー表示
2. **川情報管理**：川の名前、位置情報、説明の登録・表示
3. **ウェイポイント管理**：川の特定地点（入出艇場所、危険箇所など）の登録・表示
4. **トラック管理**：川のルート情報の登録・表示
5. **ユーザー認証**：GitHub、Facebook、Twitterを使用したOAuth認証
6. **権限管理**：一般ユーザーと管理者の権限分け
7. **アクセスログ**：ユーザーのAPIアクセスログ記録

## デプロイ環境

- Google Cloud Run上で動作
- GitHub Actionsによる自動デプロイ
- Litestreamを使用したSQLiteデータベースのGCSへのバックアップ
- カスタムドメイン設定

## 技術スタック

- **言語**：Rust
- **サーバーフレームワーク**：axum
- **フロントエンドフレームワーク**：Yew（WebAssembly）
- **地図ライブラリ**：leaflet.js（Rustラッパー）
- **データベース**：SQLite3、sqlx
- **バックアップ**：Litestream
- **認証**：axum-login、OAuth（GitHub、Facebook、Twitter）
- **テンプレートエンジン**：askama
- **ビルドツール**：trunk（WebAssembly）
- **クラウド**：Google Cloud Run、Google Cloud Storage

## 開発環境のセットアップ

### ローカル開発用の fake-gcs-server

Google Cloud Storage のエミュレーターとして fake-gcs-server を使用します。以下のコマンドで起動できます：

```bash
docker-compose up -d
```

サーバーは http://localhost:4443 で利用可能です。このエミュレーターは開発時の GCS 操作のテストに使用されます。

## cloud run 環境作成までの道のり

### サービスアカウントの作成
- ふたつ作る必要がある
- cloudrun 実行用のやつ、deployの引数にわたす
  - https://zenn.dev/nbstsh/scraps/96a5919e94ac2f
  - `roles/secretmanager.secretAccessor`
- litestream用のやつ、secret managerでマウントしてファイルで渡す
  - cloud run 環境内から gcp へアクセスするため
  - storage access(管理者)が必要
    - storage.buckets.get
    - storage.buckets.getIamPolicy
    - storage.buckets.update
    - storage.objects.create
    - storage.objects.delete
    - storage.objects.get
    - storage.objects.getIamPolicy
    - storage.objects.list
    - storage.objects.update
    - resourcemanager.projects.get
    - resourcemanager.projects.list
    - storage.managedFolders.get
    - storage.managedFolders.list


```
gcloud iam service-accounts create ...
gcloud secrets add-iam-policy-binding ...
```

### secret manager の設定

- cloud run から secret manager へアクセスするためのクレデンシャルをコンテナ内部に渡す方法
- https://blog.g-gen.co.jp/entry/secret-manager-with-cloud-run

```
gcloud secrets create ...
```

- cloud run ごとにシークレットは共通状態なので管理はデプロイとは別にしないとけない
- https://cloud.google.com/run/docs/configuring/services/secrets?hl=ja

```
gcloud run services update ... \
  --clear-secrets --clear-volumes --clear-volume-mounts --clear-env-vars
gcloud run services describe ...

```

### gar へ docker push するための設定

```
gcloud artifacts repositories create ...
gcloud auth configure-docker ...
docker build ...
docker push ...
```

### デプロイ

- 環境変数とか https://cloud.google.com/run/docs/configuring/services/secrets?hl=ja

```
gcloud run deploy ... \
  --image ... \
  --service-account ... \
  --update-env-vars=GOOGLE_APPLICATION_CREDENTIALS=/etc/key.json \
  --update-secrets=/etc/key.json=GOOGLE_APPLICATION_CREDENTIALS:1 \
  --update-secrets=FACEBOOK_CLIENT_ID=FACEBOOK_CLIENT_ID:1 \

```

### 起動時のプローブのタイムアウトの設定
```
gcloud run services describe litestream-sandbox --format export > service.yaml
```

- service.yaml をごにょごにょする
- https://cloud.google.com/run/docs/configuring/healthchecks?hl=ja

```
gcloud run services replace service.yaml
```

### custom domain mapping で dns の設定

- https://zenn.dev/mseto/articles/cloud-run-domain

## github action から deploy するための設定
- https://docs.github.com/ja/actions/security-for-github-actions/security-hardening-your-deployments/configuring-openid-connect-in-google-cloud-platform
- https://github.com/google-github-actions/deploy-cloudrun
- https://zenn.dev/cloud_ace/articles/7fe428ac4f25c8
- https://zenn.dev/marblet/articles/e61c0dcafc3dba
- Artifact Registry 書き込み
- Cloud Run 管理者
- サービス アカウント ユーザー



## Development Commands

### Local Development
```bash
# Start development servers (both frontend and backend)
./run_local.bash

# Build and check code
cargo clippy -- -D warnings
make fmt  # Format Rust code and Cargo.toml files
make check

# Database operations
cd db && sqlx migrate run  # Run migrations
./reset_local_db.bash      # Reset and recreate local database
```

### Database Management
```bash
# Reset database and apply all migrations
cd db && sqlx database reset -y

# Check database schema
sqlite3 river.db
> .mode line
> .schema
```

### Frontend Development
```bash
cd browser
trunk watch --features=local  # Development server with hot reload
trunk build --release        # Production build
```

### Git Operations
```bash
# Before pushing changes, update Claude credentials
./update_claude_credentials.bash
git push
```

### Testing
- No automated test suite is currently configured
- Manual testing via browser and API endpoints

## Architecture Overview

This is a full-stack Rust application for river mapping and information sharing in Japan.

### Workspace Structure
- **`browser/`** - Yew frontend (WebAssembly SPA)
- **`server/`** - Axum backend with OAuth authentication
- **`model/`** - Shared types between frontend/backend
- **`service/`** - Business logic layer
- **`db/`** - Database models and migrations

### Key Technologies
- **Frontend**: Yew (Rust → WebAssembly), Leaflet.js for maps
- **Backend**: Axum web framework, axum-login for OAuth
- **Database**: SQLite3 with sqlx ORM, Litestream for backup
- **Maps**: Leaflet.js with GSI tiles, OpenStreetMap layers
- **Authentication**: OAuth (GitHub, Facebook, Twitter)

### Data Model
Core entities:
- **Rivers**: Main geographic features with name, location, description
- **RiverWaypoints**: Point locations (launch spots, hazards, etc.)
- **RiverTracks**: GPS route data stored as JSON arrays
- **Users**: Authentication with role-based permissions
- **Files**: Metadata for GCS-stored content

### Database Patterns
- Use `sqlx::query!` macros for compile-time SQL checking
- Foreign key relationships ensure data integrity
- Migrations in `db/migrations/` with timestamp naming
- SQLite with JSON columns for complex data (waypoints, tracks)

### Frontend Architecture
- Component-based structure in `browser/src/components/`
- Shared state management via Yew hooks
- API calls through gloo-net to backend endpoints
- Map integration using leaflet-rs wrapper

### Authentication Flow
- OAuth providers: GitHub, Facebook, Twitter
- Session management with secure cookies
- Role-based access (admin/user permissions)
- Service account for GCS operations

### Deployment
- Google Cloud Run containerized deployment
- Litestream for SQLite backup to GCS
- Terraform infrastructure management
- GitHub Actions for CI/CD

## Code Investigation Guidelines

### Efficient Source Code Investigation
- **Avoid reading entire files one by one** - use targeted searches instead
- Use `cargo modules structure -p <crate_name>` to understand module structure before reading files
- Always read README.md files first for context
- Use search tools effectively:
  - `git grep "pub fn function_name"` for public functions in Rust
  - `rg function_name` for general function searches
  - `git ls-files` to list repository files

### Rust Code Quality Standards
- **Scoped use statements**: Place `use` statements inside function scope rather than at file top
  - Prevents namespace pollution and clarifies dependencies
  - Use fully qualified paths for types (e.g., `sqlx::sqlite::SqliteConnection`)
  - Prefer `std::result::Result` over library-specific result types like `anyhow::Result`
- **Async functions**: Functions with side effects must be `async fn` or return `impl Future`

### Database Operations
- Use `sqlx::query!` macros for compile-time SQL checking
- SQLite database operations through Docker MCP server when available
- Available MCP tools: `list_tables`, `read_query`, `write_query`, `create_table`, `describe_table`

## Development Patterns

### Adding New Rivers/Waypoints
- Frontend forms in `add_river.rs`, `add_waypoint.rs`
- API endpoints in `server/src/web/api.rs`
- Service layer implementation in `service/`
- Database operations use sqlx macros

### Map Integration
- Leaflet instance managed in `map.rs` component
- Multiple layer support (GSI, OSM, aerial photos)
- Coordinate handling with lat/lng precision

### File Storage
- Google Cloud Storage integration
- Metadata stored in database `files` table
- Service account authentication for uploads

## Local Development Setup

### Prerequisites
- Rust toolchain (see rust-toolchain.toml)
- SQLite3
- Node.js (for leaflet dependency)

### Environment
```bash
# For local development, fake-gcs-server runs on http://localhost:4443
docker-compose up -d

# Environment variables for local development use "local" feature flag
```

### OAuth Configuration
Set up OAuth apps for development:
- GitHub: https://github.com/settings/applications/
- Facebook: https://developers.facebook.com/
- Twitter: https://developer.twitter.com/


## 利用可能なMCPツール

- **sqlite**：SQLiteデータベースに対してクエリを実行するツール
  - list_tables：データベース内のテーブル一覧を取得
  - read_query：SELECT文を実行してデータを取得
  - write_query：INSERT、UPDATE、DELETE文を実行
  - create_table：テーブルを作成
  - describe_table：テーブルの構造を取得

## ソースコードの調査方法

- **なるべくファイルをひとつひとつ全体を読まないこと**
- crate のファイルの中身を読む前、編集する前に、まず cargo modules コマンドでモジュールの構造を把握すること
   - ex. `cargo modules structure -p <crate name>`:  crate name のモジュール構造を出力
- **README.mdを読むこと**
- `git grep "pub fn 関数名"`：公開されている特定の関数名を含むコードを検索
   - rust なので有効
- `rg 関数名`：特定の関数名を含むコードを検索
- `git ls-files`: リポジトリ内のファイル一覧を取得

## Rustライブラリの調査方法

### docs.rsを使用したドキュメント検索

docs.rs（Rustの公式ドキュメントサイト）から任意のRustライブラリのドキュメントを読むことができます。以下のような方法で必要な情報を検索できます：

1. **ライブラリのドキュメント検索**：`web_search`を使用して「site:docs.rs ライブラリ名」と検索
2. **特定の関数やモジュールの検索**：`web_search`を使用して「site:docs.rs ライブラリ名 関数名」と検索
3. **ドキュメントの閲覧**：`fetch`を使用して検索結果のURLからドキュメントを取得

#### tokio APIの使い方の検索方法の例

tokioの非同期ランタイムやI/O操作について調べる場合：

```rust
// tokioのランタイムを使用した基本的な非同期プログラム
#[tokio::main]
async fn main() {
    // 非同期タスクの作成
    let handle = tokio::spawn(async {
        // 非同期処理
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        "Hello from async task"
    });
    
    // タスクの完了を待機
    let result = handle.await.unwrap();
    println!("{}", result);
}
```

tokioの特定の機能（例：ファイルI/O）について調べるには：

1. `web_search`で「site:docs.rs tokio fs」と検索
2. 検索結果から適切なドキュメントページを見つける
3. そのページを取得して詳細を確認


### GitHubリポジトリからのソースコード調査

公式ドキュメントだけでなく、GitHubリポジトリからソースコード、README、examplesなどを直接調査することも重要です。以下の方法で効率的に調査できます：

1. **リポジトリの検索**：`web_search`を使用して「github ライブラリ名 repository」と検索
2. **READMEの確認**：リポジトリのREADME.mdを取得し、基本的な使い方や設計思想を理解
3. **examplesディレクトリの調査**：リポジトリ内のexamplesディレクトリにある実際の使用例を確認
4. **テストコードの確認**：テストコードは実際の使用例として参考になることが多い

## Rustコード品質向上のポイント

1. **関数スコープ内でのuseステートメントの使用**
   - ファイル先頭のuseステートメントを避け、関数スコープ内でuseステートメントを使用する
   - メリット: 名前空間の汚染を防ぎ、依存関係をより明確にする
   - 実装方法:
     - ファイル先頭のuseステートメントを削除
     - 各関数内で必要なuseステートメントを追加（例: `use anyhow::Result;`）
     - 型名を完全修飾パスで指定（例: `sqlx::sqlite::SqliteConnection`）
   - ライブラリの提供する anyhow::Result などの使用は避けて,  prelude で import されている std::resukt::Result のみを使用すること
   - 例:
     ```rust
     // 修正前
     use anyhow::Result;
     use sqlx::sqlite::SqliteConnection;
     
     pub async fn get_data(conn: &mut SqliteConnection) -> Result<Vec<Data>> {
         // 関数の実装
     }
     
     // 修正後
     pub async fn get_data(conn: &mut sqlx::sqlite::SqliteConnection) -> Result<Vec<Data>, anyhow::Error> {
         use anyhow::Result;
         // 関数の実装
     }
     ```

2. 副作用を扱う間数は必ず async fn または fn() -> impl Future の間数にすること
