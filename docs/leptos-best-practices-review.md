# river.duxca.com — Leptos ベストプラクティス照合レポート

作成日: 2026-06-12

前回整理した Leptos ベストプラクティスと、このプロジェクトの現状を照合した結果です。

## 総評

| 観点 | 評価 |
|---|---|
| ワークスペース・`cargo-leptos` 設定 | 良い |
| Server Functions + `shared-api` | 良い |
| WASM 最適化プロファイル | 良い |
| UI の共有クレート設計 | 外れている |
| SSR + Hydration の一貫性 | 外れている |
| 移行途中の二重フロントエンド | 許容範囲（ただし技術的負債） |

---

## 重大: SSR と Hydration の不整合（`/app`）

`/app` ルートは SSR の仕組みを使っていますが、実際には CSR（クライアントサイドマウント）になっています。

### サーバー側

`server/src/web/app.rs` では `<body>` が空です。

```rust
let handler = leptos_axum::render_app_to_stream(move || {
    view! {
        <!DOCTYPE html>
        <html lang="ja">
            // ...
            <body></body>
        </html>
    }
});
```

### クライアント側

`leptos-browser/src/lib.rs` では `hydrate()` という名前ですが、`mount_to_body` で新規マウントしています。

```rust
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
```

`HydrationScripts` を出力しているのに、サーバーが `App` をレンダリングしていないため、**SSR + Hydration ではなく「空のシェル + CSR マウント」** です。

### ベストプラクティスとの差

- **SSR + Hydration なら**: サーバーで `<App/>` を描画し、クライアントで `hydrate_body(App)` する
- **CSR なら**: `HydrationScripts` や `render_app_to_stream` は不要で、静的 HTML + スクリプト読み込みで十分

初回表示の遅延や SEO の面で不利になります。

---

## 重大: 共有 `app` クレートがない（UI の二重管理）

公式テンプレートは `app`（共有 UI）+ `frontend`（WASM エントリ）+ `server` ですが、このプロジェクトは次のように分かれています。

| 場所 | 内容 | SSR | Hydration |
|---|---|---|---|
| `server/src/web/ui.rs` | ホーム・ログインページ | あり | なし |
| `leptos-browser/src/lib.rs` | 地図アプリ `App` | なし | CSR マウント |

`server` は `leptos-browser` を依存に入れておらず、`leptos-browser` にも `ssr` feature がありません。そのため **同じコンポーネントをサーバーとクライアントで共有できない** 状態です。

`server/src/web/home.rs` は正しい SSR パターンですが、コンポーネントが `server` クレート内に閉じています。

```rust
let handler = leptos_axum::render_app_to_stream_with_context(
    || {},
    move || {
        view! {
            <crate::web::ui::HomePage user=user.clone() providers=providers.clone()/>
        }
    },
);
```

Yew からの移行途中なら理解できますが、長期的には `app` クレートに UI を集約するのがベストプラクティスです。

---

## 中程度: `shared-api` が未使用

`leptos-browser` は `shared-api` に依存していますが、`App` 内で Server Functions（`list_rivers` など）を呼んでいません。地図はハードコードされた座標のみです。

```rust
const FUJI_RIVER: Position = Position {
    lat: 35.362_222,
    lng: 138.731_389,
};
```

API 層は整備済みなのに UI が未接続で、**インフラだけ先にできている状態**です。

---

## 中程度: `leptos_router` がない

クライアント側ルーティングがありません。地図アプリが `/app` 単一ページのままなので、Yew 版のような複数画面構成に戻すには `leptos_router` の導入が必要です。

---

## 中程度: 依存バージョンの不統一

```toml
# leptos-browser
leptos = { version = "0.8", default-features = false }

# server / shared-api
leptos = { version = "0.8.6", features = ["ssr"] }
```

`workspace.dependencies` でバージョンを固定するのが推奨です。パッチ違いで SSR と WASM 側の挙動差が出る可能性があります。

---

## 中程度: スタイルの二重管理

- 地図アプリ: `leptos-browser/style.css`（`cargo-leptos` 管理）
- ホーム/ログイン: `server/src/web/ui.rs` 内のインライン `<style>`

レイアウトやデザインを統一しづらく、メンテナンスコストが上がります。

---

## 軽微: その他の差分

### 許容範囲（移行中）

- Yew の `browser/` と Leptos の `leptos-browser/` が併存（`browser/` はワークスペース外なので影響は限定的）
- E2E が Playwright ではなく fantoccini/WebDriver（独自方式としては問題なし）

### 改善余地あり

- `assets-dir` が `cargo-leptos` 設定にない（マーカー画像などを配るなら必要）
- `.fallback(home)` で未知ルートもホームに落ちる（404 として扱わない）
- ルーター全体に `CorsLayer::very_permissive()` がかかっている
- `leptos-browser` に `components/` や `pages/` の分割がまだない（プロトタイプ段階なら許容）

---

## できていること（ベストプラクティスに沿っている）

これらはそのまま維持してよい部分です。

1. **`cargo-leptos` ワークスペース設定** — `lib-package` / `bin-package` / `wasm-release` が適切
2. **`shared-api` による Server Functions 分離** — `#[server]` + `provide_context` パターンがきれい
3. **既存 `service` / `model` レイヤーの再利用** — 大規模アプリ向けの良い設計
4. **feature フラグ設計** — `shared-api` の `ssr` / `hydrate` 分離
5. **認証付き Server Function ハンドラ** — Axum 層で認証してから `handle_server_fns_with_context`

---

## 優先度付き改善案

| 優先度 | 対応 |
|---|---|
| 高 | `leptos-browser` に `ssr` feature を追加し、`app_shell` で `<App/>` を SSR レンダリング + `hydrate_body` に切り替え |
| 高 | 共有 `app` クレートを作り、`server` の `ui.rs` と `leptos-browser` の UI を統合 |
| 中 | `App` から `list_rivers` 等の Server Functions を `Resource` で呼ぶ |
| 中 | `leptos_router` でルーティング導入 |
| 低 | `workspace.dependencies` でバージョン統一、スタイル統合 |

---

## まとめ

移行途中としては「外れている」とまでは言い切れない部分もありますが、**いちばん気にすべきは `/app` の SSR/Hydration の不整合と、UI が `server` と `leptos-browser` に分断されている点**です。ここを直すと、残りの移行作業もスムーズになります。

---

## 参考リンク

- [Leptos Book（公式）](https://book.leptos.dev/)
- [start-axum-workspace テンプレート](https://github.com/leptos-rs/start-axum-workspace)
- [Islands ガイド](https://book.leptos.dev/islands.html)
- [SSR デプロイ](https://book.leptos.dev/deployment/ssr.html)
- [Global State](https://book.leptos.dev/15_global_state.html)
- [Server Function Extractors](https://book.leptos.dev/server/26_extractors.html)