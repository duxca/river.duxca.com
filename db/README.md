### sqlx::query!マクロの使い方の例

sqlxのコンパイル時クエリチェック機能を使用する例：

```rust
// 匿名レコード型を返すquery!マクロ
let rivers = sqlx::query!(
    r#"
    SELECT river_id, river_name, waypoint
    FROM rivers
    WHERE user_id = ?
    "#,
    user_id
)
.fetch_all(&pool) // -> Vec<{ river_id: i64, river_name: String, waypoint: serde_json::Value }>
.await?;

// 名前付き構造体にマッピングするquery_as!マクロ
#[derive(sqlx::FromRow)]
struct River {
    river_id: i64,
    river_name: String,
    waypoint: serde_json::Value
}

let rivers = sqlx::query_as!(River,
    r#"
    SELECT river_id, river_name, waypoint
    FROM rivers
    WHERE user_id = ?
    "#,
    user_id
)
.fetch_all(&pool) // -> Vec<River>
.await?;
```

#### sqlx::query!マクロとsqlx::query関数の違い

sqlx::query!マクロとsqlx::query関数の主な違いは以下の通りです：

1. **コンパイル時チェック**: query!マクロはコンパイル時にSQLの構文と型をチェックしますが、query関数は実行時にのみチェックします
2. **型安全性**: query!マクロは結果の型を自動的に推論し、型安全なアクセスを提供しますが、query関数は手動でRowから値を取得する必要があります
3. **パフォーマンス**: query!マクロはコンパイル時に型情報を生成するため、実行時のオーバーヘッドが少なくなります
4. **エラー検出**: query!マクロはコンパイル時にSQLエラーを検出できますが、query関数は実行時にのみエラーを検出します

#### sqlx::query!マクロとsqlx::query_as!マクロの違い

1. **query!マクロ**: 匿名レコード型を返します。カラム名がそのままフィールド名になります
2. **query_as!マクロ**: 指定した構造体にマッピングします。構造体のフィールド名とカラム名が一致する必要があります

マイグレーション実行前でも、sqlx::query!マクロを使用することが推奨されます：

```rust
// マイグレーション実行前でもマクロを使用
let river_id = sqlx::query!(
    r#"
    INSERT INTO rivers (user_id, river_name) VALUES (?, ?) RETURNING river_id
    "#,
    user_id,
    river_name
)
.fetch_one(&mut conn)
.await?
.river_id;
```

## データベース操作のTips

### マイグレーションとデータベース管理

1. **データベースのリセットと初期化**
   - `sqlx database reset`コマンドを使用してデータベースをリセットし、マイグレーションを再適用できます
   - このコマンドは、データベースを削除して再作成し、すべてのマイグレーションを適用します

2. **マイグレーションファイルの直接適用**
   - マイグレーションファイルを直接SQLiteに適用することもできます
   - 例：`sqlite3 river.db < migrations/20240729163101_users.sql`

3. **SQLファイルの修正**
   - sedコマンドを使用してSQLファイルを一括修正できます
   - 例：
     ```bash
     # riversテーブルへの挿入文にuser_idカラムとdescriptionカラムを追加
     sed -E 's/INSERT INTO rivers \(river_name, waypoint\) VALUES \(/INSERT INTO rivers \(user_id, river_name, waypoint, description\) VALUES \(1, /g; s/\)\);/\), ""\);/g' seed.sql > fixed_seed.sql
     ```

### データベース設計とマイグレーション

1. **リレーショナルデータモデリング**
   - 外部キー制約を使用した関連テーブル間の整合性確保
   - メタデータテーブルの設計（例: river_waypoint_images）
   - 適切なインデックス設計によるクエリパフォーマンスの最適化

2. **マイグレーション管理**
   - 命名規則: `db/migrations/YYYYMMDDHHMMSS_テーブル名.sql`
   - 実行方法: `cd db && sqlx migrate run`
   - ロールバック考慮: DOWN migrationの実装
