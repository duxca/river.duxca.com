## 初回デプロイ

1. `./cli/reset_remote_db.bash`
1. `./cli/deploy.bash`

## 運用

1. `./cli/deploy.bash`


## tips

### river

- https://japan-safe-paddling.org/link/
- https://www.japanriversup.com/%E3%82%BB%E3%83%BC%E3%83%95%E3%83%86%E3%82%A3/%E5%B7%9D%E3%82%92%E7%9F%A5%E3%82%8B-%E6%B5%81%E3%82%8C%E3%82%92%E7%9F%A5%E3%82%8B/
- https://funq.jp/blades/

### 地理院地図
- https://maps.gsi.go.jp/development/sample.html
- https://github.com/gsi-cyberjapan/gsitiles-leaflet/blob/gh-pages/index.html
- https://www.gsi.go.jp/kohokocho/map-sign-tizukigou-2022-itiran.html
- https://maps.gsi.go.jp/development/ichiran.html
- https://user.numazu-ct.ac.jp/~tsato/webmap/map/lmap.html
- https://github.com/gsi-cyberjapan/gsitiles-leaflet/blob/gh-pages/index.html
- https://kurage.ready.jp/w_map/ex-opn.html#smpl07

### litestream
- https://qiita.com/ydclab_0006/items/9503303f7f3112dc760a
- https://cloud.google.com/run/docs/configuring/secrets?hl=ja
- https://litestream.io/guides/gcs/


### leaflet
- https://github.com/slowtec/leaflet-rs/blob/master/examples/yew-component/index.html
- https://qiita.com/SAmmys/items/5d187f6c5be3d398f9e8
- https://leafletjs.com/download.html
- https://github.com/slowtec/leaflet-rs/pull/34

### openlayers
- https://openlayers.org/workshop/en/mobile/geolocation.html
- https://openlayers.org/en/latest/apidoc/module-ol_Feature-Feature.html

### yew
- https://yew.rs/docs/concepts/basic-web-technologies/js
- https://yew.rs/docs/concepts/basic-web-technologies/html
- https://rustwasm.github.io/docs/wasm-bindgen/
- https://yew.rs/ja/docs/tutorial#fetching-data-using-external-rest-api
- https://docs.rs/gloo-net/latest/gloo_net/http/struct.Response.html
- https://rustwasm.github.io/wasm-bindgen/api/js_sys/index.html
- https://docs.rs/gloo-utils/latest/gloo_utils/format/trait.JsValueSerdeExt.html
- https://docs.rs/crate/gloo/latest/features
- https://ja.react.dev/reference/react/useEffect
- https://github.com/yewstack/yew/issues/3563
- https://docs.rs/yew-hooks/latest/yew_hooks/prelude/index.html
- https://yew.rs/docs/next/concepts/function-components/hooks
- https://yew.rs/docs/next/tutorial#fetching-data-using-external-rest-api
- https://yew-rs-api.web.app/next/yew/functional/
- https://docs.rs/serde-wasm-bindgen/latest/serde_wasm_bindgen/
- https://rustwasm.github.io/wasm-bindgen/reference/types/jsvalue.html
- https://rustwasm.github.io/wasm-bindgen/reference/types/result.html
- https://yew.rs/docs/next/more/debugging
- https://github.com/jetli/yew-hooks
- https://legacy.reactjs.org/docs/hooks-effect.html
- https://yew-rs-api.web.app/next/yew/functional/fn.use_effect_with.html
- https://yew.rs/ja/docs/next/concepts/html/conditional-rendering
- https://zenn.dev/uhyo/articles/useeffect-taught-by-extremist

### trunk
- https://trunkrs.dev/guide/assets/index.html
- https://trunkrs.dev/assets/

### axum
- https://github.com/maxcountryman/axum-login
- https://github.com/maxcountryman/tower-sessions
- https://github.com/tokio-rs/axum

### askama
- https://djc.github.io/askama/template_syntax.html

### sqlx
- https://github.com/launchbadge/sqlx

### oauth
- https://docs.github.com/ja/apps/oauth-apps/building-oauth-apps/authorizing-oauth-apps
- https://docs.github.com/ja/apps/oauth-apps/maintaining-oauth-apps/troubleshooting-oauth-app-access-token-request-errors#incorrect-client-credentials
- https://docs.github.com/en/rest/overview/resources-in-the-rest-api?apiVersion=2022-11-28#user-agent-required
- https://developers.facebook.com/docs/facebook-login/guides/advanced/manual-flow/?locale=ja_JP#confirm
- https://developers.facebook.com/docs/instagram-basic-display-api/reference/me?locale=ja_JP
- https://apidog.com/jp/blog/facebook-oauth-2-auth-process/
- https://developers.facebook.com/docs/instagram-basic-display-api/reference/user?locale=ja_JP

### gcp
- https://cloud.google.com/identity-platform/pricing?hl=ja#identity-platform-pricing
- https://support.google.com/analytics/answer/9304153?hl=ja#zippy=%2C%E3%82%A6%E3%82%A7%E3%83%96%E3%82%B5%E3%82%A4%E3%83%88%E4%BD%9C%E6%88%90%E3%83%84%E3%83%BC%E3%83%AB%E3%81%BE%E3%81%9F%E3%81%AF-cms-%E3%81%A7%E3%83%9B%E3%82%B9%E3%83%88%E3%81%95%E3%82%8C%E3%82%8B%E3%82%A6%E3%82%A7%E3%83%96%E3%82%B5%E3%82%A4%E3%83%88hubspotshopify-%E3%81%AA%E3%81%A9%E3%81%AB%E3%82%BF%E3%82%B0%E3%82%92%E8%BF%BD%E5%8A%A0%E3%81%99%E3%82%8B

### sqlite3
- https://ytyaru.hatenablog.com/entry/2021/05/26/000000
- https://ytyaru.hatenablog.com/entry/2021/06/22/000000
- https://www.sqlite.org/pragma.html#pragma_auto_vacuum
- https://blog.jnito.com/entry/2023/05/23/104124
- https://zenn.dev/sql_geinin/books/9e9fb9492c54f6/viewer/d9c00a


```
$ sqlite3 river.db
> .mode line
> .schema
```

```
sqlite3 app.sqlite3 < seeds.sql
```

## accounts

- https://litestream-sandbox-4h2uh5x4wa-an.a.run.app
- https://developers.facebook.com/apps/461498540066147/dashboard/
- https://github.com/settings/applications/2657880
- https://dash.cloudflare.com/688933c4553b4284a2684583893badc9/domains/duxca.com
- 


