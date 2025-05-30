# Roo への指示
## 利用可能なMCPツール

- **brave-search**：インターネット検索を行うためのツール
  - brave_web_search：一般的なウェブ検索を実行
  - brave_local_search：ローカルビジネスや場所の検索を実行
- **fetch**：インターネット上のURLからコンテンツを取得するツール
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

MCPのbrave-searchとfetchツールを使用して、docs.rs（Rustの公式ドキュメントサイト）から任意のRustライブラリのドキュメントを読むことができます。以下のような方法で必要な情報を検索できます：

1. **ライブラリのドキュメント検索**：`brave_web_search`を使用して「site:docs.rs ライブラリ名」と検索
2. **特定の関数やモジュールの検索**：`brave_web_search`を使用して「site:docs.rs ライブラリ名 関数名」と検索
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

1. `brave_web_search`で「site:docs.rs tokio fs」と検索
2. 検索結果から適切なドキュメントページを見つける
3. `fetch`でそのページを取得して詳細を確認


### GitHubリポジトリからのソースコード調査

公式ドキュメントだけでなく、GitHubリポジトリからソースコード、README、examplesなどを直接調査することも重要です。以下の方法で効率的に調査できます：

1. **リポジトリの検索**：`brave_web_search`を使用して「github ライブラリ名 repository」と検索
2. **READMEの確認**：`fetch`を使用してリポジトリのREADME.mdを取得し、基本的な使い方や設計思想を理解
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