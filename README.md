
![workflow](https://github.com/legokichi/river.duxca.com/actions/workflows/rust.yml/badge.svg)


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
  --clear-secrets
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

## tips

### river

- https://japan-safe-paddling.org/link/
- https://www.japanriversup.com/%E3%82%BB%E3%83%BC%E3%83%95%E3%83%86%E3%82%A3/%E5%B7%9D%E3%82%92%E7%9F%A5%E3%82%8B-%E6%B5%81%E3%82%8C%E3%82%92%E7%9F%A5%E3%82%8B/
- https://funq.jp/blades/
- https://funq.jp/blades/article/529433/#%E6%B9%98%E5%8D%97%EF%BC%8F%E6%9D%90%E6%9C%A8%E5%BA%A7
- https://sxsoutdoor.com/spot-nakahinukakakou/#:~:text=%E6%B6%B8%E6%B2%BC%E5%B7%9D%E5%8F%8A%E3%81%B3%E9%82%A3%E7%8F%82%E5%B7%9D%E3%81%A7%E3%81%AF,%E3%81%A6%E8%89%AF%E3%81%84%E3%81%A8%E3%81%AE%E3%81%93%E3%81%A8%E3%80%82
- https://supyuki.wordpress.com/2019/04/03/sup%E4%BA%BA%E5%8F%A3%E3%81%8C%E5%A4%9A%E3%81%84%E3%81%AE%E3%81%AF%E3%81%A9%E3%81%93%E3%81%A0%EF%BC%9F%EF%BC%8Fsupa%E4%BC%9A%E5%93%A1%E3%83%BB%E5%9C%B0%E5%9F%9F%E5%88%A5%E6%A7%8B%E6%88%90%E6%AF%942019/
- https://www.amazon.co.jp/%E6%97%A5%E6%9C%AC%E3%81%AE%E5%B7%9D%E5%9C%B0%E5%9B%B3101-%E3%82%AB%E3%83%8C%E3%83%BC%E3%83%BB%E3%83%84%E3%83%BC%E3%83%AA%E3%83%B3%E3%82%B0%E3%83%9E%E3%83%83%E3%83%97%E2%80%95%E3%82%AB%E3%83%8C%E3%83%BC%E3%83%BB%E3%83%84%E3%83%BC%E3%83%AA%E3%83%B3%E3%82%B0%E3%83%BB%E3%83%9E%E3%83%83%E3%83%97-BE-PAL-OUTING-SPECIAL/dp/4091046819
- https://www.amazon.co.jp/%E5%85%A8%E5%9B%BD%E3%83%AA%E3%83%90%E3%83%BC%E3%83%84%E3%83%BC%E3%83%AA%E3%83%B3%E3%82%B055%E3%83%9E%E3%83%83%E3%83%97-Outdoor/dp/463550025X
- https://www.amazon.co.jp/%E9%9B%91%E8%AA%8C-%E3%82%AB%E3%83%8C%E3%83%BC%E3%83%A9%E3%82%A4%E3%83%95-%E6%9C%AC/s?rh=n%3A13384021%2Cp_lbr_one_browse-bin%3A%E3%82%AB%E3%83%8C%E3%83%BC%E3%83%A9%E3%82%A4%E3%83%95
- https://www.kawa-asobi.net/book/20161208_3820
- https://slackline.jp/packraft/post-26857/
- https://www2u.biglobe.ne.jp/~hiro-ito/library/canoebook.htm
- https://magazine.tsuritickets.com/2020/01/08/%E6%B8%93%E6%B5%81%E9%87%A3%E3%82%8A%E3%81%AB%E3%81%AF%E3%82%B9%E3%83%9E%E3%83%9B%E3%82%A2%E3%83%97%E3%83%AA%E3%81%8C%E3%81%82%E3%82%8B%E3%81%A8%E4%BE%BF%E5%88%A9%EF%BC%81%E3%81%8A%E3%81%99%E3%81%99/
- https://river.longseller.org/t/4424.html
- https://www.river.go.jp/kawabou/mb?zm=11&clat=35.87736716144893&clon=139.6238708496094&mapType=0&viewGrpStg=0&viewRd=1&viewRW=1&viewRiver=1&viewPoint=1&fld=0
- https://www.river.or.jp/koeki/opendata/index.html
- https://en.wikipedia.org/wiki/International_scale_of_river_difficulty
- https://gopaddling.info/river-gradings-simple-guide/
- https://en.wikipedia.org/wiki/Rapids


### 多摩川
- https://www.kaifugun-yamakawacho.net/canoe/suiikei00/mysuiikei01ac.htm
- https://tama-river.kaifugun-yamakawacho.net/
- https://www.kaifugun-yamakawacho.net/canoe/canoe00.htm

### 地理院地図
- https://maps.gsi.go.jp/development/sample.html
- https://github.com/gsi-cyberjapan/gsitiles-leaflet/blob/gh-pages/index.html
- https://www.gsi.go.jp/kohokocho/map-sign-tizukigou-2022-itiran.html
- https://maps.gsi.go.jp/development/ichiran.html
- https://user.numazu-ct.ac.jp/~tsato/webmap/map/lmap.html

### GIS
- https://qiita.com/yabooun/items/da59e47d61ddff141f0c
- https://gis-oer.github.io/gitbook/book/materials/web_gis/GeoJSON/GeoJSON.html#%E3%82%B9%E3%82%BF%E3%82%A4%E3%83%AA%E3%83%B3%E3%82%B0

### litestream
- https://qiita.com/ydclab_0006/items/9503303f7f3112dc760a
- https://cloud.google.com/run/docs/configuring/seets?hl=ja
- https://litestream.io/guides/gcs/
- https://zenn.dev/voluntas/scraps/f4939cbe92525c
- https://zenn.dev/oubakiou/articles/382839bfc65931
- https://qiita.com/faable01/items/ac7418d671c6db5b966f
- https://qiita.com/hide_seki/items/f18a6b4d788738b3f8e4

### ジオグラフィカ
- https://note.com/keizi666/

### leaflet
- https://github.com/slowtec/leaflet-rs/blob/master/examples/yew-component/index.html
- https://qiita.com/SAmmys/items/5d187f6c5be3d398f9e8
- https://leafletjs.com/download.html
- https://github.com/slowtec/leaflet-rs/pull/34
- https://qiita.com/poruruba/items/88e23011815e8e0f4ffb
- https://kurage.ready.jp/w_map/ex-opn.html#smpl07

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
- https://zenn.dev/pyhrinezumi/articles/8455f0d61e856f
- https://developers.play.jp/entry/2024/05/10/162819
- https://docs.x.com/resources/fundamentals/authentication/oauth-2-0/authorization-code
- https://zenn.dev/nyancat/articles/20230803-twitter-api-oauth2-pkce

### gcp
- https://cloud.google.com/identity-platform/pricing?hl=ja#identity-platform-pricing
- https://support.google.com/analytics/answer/9304153?hl=ja#zippy=%2C%E3%82%A6%E3%82%A7%E3%83%96%E3%82%B5%E3%82%A4%E3%83%88%E4%BD%9C%E6%88%90%E3%83%84%E3%83%BC%E3%83%AB%E3%81%BE%E3%81%9F%E3%81%AF-cms-%E3%81%A7%E3%83%9B%E3%82%B9%E3%83%88%E3%81%95%E3%82%8C%E3%82%8B%E3%82%A6%E3%82%A7%E3%83%96%E3%82%B5%E3%82%A4%E3%83%88hubspotshopify-%E3%81%AA%E3%81%A9%E3%81%AB%E3%82%BF%E3%82%B0%E3%82%92%E8%BF%BD%E5%8A%A0%E3%81%99%E3%82%8B
- https://zenn.dev/collabostyle/articles/89a9171ab0c0e5

### sqlite3
- https://ytyaru.hatenablog.com/entry/2021/05/26/000000
- https://ytyaru.hatenablog.com/entry/2021/06/22/000000
- https://www.sqlite.org/pragma.html#pragma_auto_vacuum
- https://blog.jnito.com/entry/2023/05/23/104124
- https://zenn.dev/sql_geinin/books/9e9fb9492c54f6/viewer/d9c00a
- https://soudai.hatenablog.com/entry/2018/05/01/204442

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


