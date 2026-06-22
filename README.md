[![workflow](https://github.com/legokichi/river.duxca.com/actions/workflows/check.yml/badge.svg)](https://github.com/legokichi/river.duxca.com/actions/workflows/check.yml)
[![deploy](https://github.com/legokichi/river.duxca.com/actions/workflows/deploy.yml/badge.svg)](https://github.com/legokichi/river.duxca.com/actions/workflows/deploy.yml)

## 開発環境のセットアップ

### ローカルの hot reload

`cargo-leptos` が Axum server と Leptos/WASM frontend をまとめて watch します。Axum と frontend はどちらも `127.0.0.1:18080` で配信されます。

初回だけ `cargo-leptos` と `sqlx-cli` を入れます。

```bash
make setup
```

起動:

```bash
make serve
```

ブラウザで確認する URL:

```text
http://127.0.0.1:18080/       # Axum のログイン確認 UI
http://127.0.0.1:18080/login  # ログイン UI
http://127.0.0.1:18080/app    # hot reload 付き Leptos frontend
```

### テスト

サーバ、WASM frontend、e2e crate のコンパイル確認:

```bash
make check
make build
```

Rust/fantoccini のシナリオテストは次のコマンドで実行します。内部では `cargo leptos end-to-end` がローカルサーバを起動し、e2e用コンテナ内の `chromedriver` に `fantoccini` が接続します。

```bash
make test-e2e
```

通常の `cargo test` では WebDriver が必要な smoke test は実行しません。

現在の smoke test は以下を確認します。

- `/` にログイン導線が表示されること
- `/login` にプロバイダボタンが表示されること
- `/app` でLeptos app shellとWASM frontend bundleが配信されること

## 本番デプロイ

Terraform は役割ごとに state を分けています。

- `terraform_bootstrap`: 各 Terraform stack の remote state 用 GCS bucket。bootstrap 循環を避けるため local state で管理する。
- `terraform_gcp_storage`: GCP の基盤。API、Artifact Registry、Litestream GCS bucket、Secret Manager secret 本体、Cloud Run service account、IAM。
- `terraform_gcp_app`: Cloud Run service、public access、`river.duxca.com` の Cloud Run domain mapping。
- `terraform_ci`: GitHub Actions 用 service account、Workload Identity Federation、deploy IAM。
- `terraform_cf`: Cloudflare DNS record。

初回デプロイ順は固定です。remote state bucket がないと他 stack を初期化できず、Artifact Registry repository がないと container image を push できず、container image と secret version がないと Cloud Run を作れないためです。

### 1. 認証

```bash
gcloud auth application-default login
gcloud config set project river-duxca-prod
```

ADC を使えない環境では次を使います。

```bash
export GOOGLE_OAUTH_ACCESS_TOKEN="$(gcloud auth print-access-token)"
```

### 2. Terraform bootstrap stack

Remote state 用 GCS bucket を作ります。`terraform_ci` 自身もこの bucket を backend として使うため、state bucket 管理は `terraform_ci` には含めません。

```bash
terraform -chdir=terraform_bootstrap init
terraform -chdir=terraform_bootstrap plan -var-file=prod.tfvars
terraform -chdir=terraform_bootstrap apply -var-file=prod.tfvars
```

既存 bucket を後から Terraform 管理下に置く場合は、先に import します。

```bash
terraform -chdir=terraform_bootstrap import \
  -var-file=prod.tfvars \
  google_storage_bucket.terraform_state \
  river-duxca-prod-terraform-state
```

### 3. GCP storage stack

Artifact Registry、Litestream bucket、Secret Manager secret、service account を作ります。

```bash
terraform -chdir=terraform_gcp_storage init -reconfigure
terraform -chdir=terraform_gcp_storage plan -var-file=prod.tfvars
terraform -chdir=terraform_gcp_storage apply -var-file=prod.tfvars
```

### 4. OAuth client ID と secret を設定する

OAuth の `Client ID` は公開識別子なので `terraform_gcp_app/prod.tfvars` に置きます。`Client Secret` だけ Secret Manager に入れます。Terraform は secret の入れ物と IAM だけを管理し、値は version として手で入れます。

GitHub OAuth の値は GitHub の OAuth App で作ります。

1. GitHub の `Settings` → `Developer settings` → `OAuth Apps` → `New OAuth App` を開く。
2. `Application name`: `river.duxca.com`
3. `Homepage URL`: `https://river.duxca.com`
4. `Authorization callback URL`: `https://river.duxca.com/oauth/callback/github`
5. 作成後に表示される `Client ID` を `terraform_gcp_app/prod.tfvars` の `github_client_id` に設定する。
6. `Generate a new client secret` で作った値を `GITHUB_CLIENT_SECRET` secret version に入れる。

Facebook OAuth の値も同じく Facebook Developers 側で app を作り、callback URL は `https://river.duxca.com/oauth/callback/facebook` にします。`App ID` は `terraform_gcp_app/prod.tfvars` の `facebook_client_id`、`App Secret` は `FACEBOOK_CLIENT_SECRET` secret version に入れます。

```bash
printf '%s' "$FACEBOOK_CLIENT_SECRET" |
  gcloud secrets versions add FACEBOOK_CLIENT_SECRET \
    --project=river-duxca-prod \
    --data-file=-

printf '%s' "$GITHUB_CLIENT_SECRET" |
  gcloud secrets versions add GITHUB_CLIENT_SECRET \
    --project=river-duxca-prod \
    --data-file=-
```

Cloud Run は各 client secret の version `1` を参照します。値を入れ直して version が増えた場合は、Terraform 側の `secret_key_ref.key` も合わせて更新してください。

### 5. Container image を push

```bash
IMAGE_REPOSITORY="asia-northeast1-docker.pkg.dev/river-duxca-prod/cloud-run-source-deploy/litestream-sandbox"

gcloud auth configure-docker asia-northeast1-docker.pkg.dev

docker buildx build . \
  --tag="${IMAGE_REPOSITORY}:latest" \
  --push \
  --metadata-file=/tmp/river-image-metadata.json

IMAGE_DIGEST="$(jq -r '."containerimage.digest"' /tmp/river-image-metadata.json)"
CONTAINER_IMAGE="${IMAGE_REPOSITORY}@${IMAGE_DIGEST}"
```

### 6. 必要なら Litestream backup を restore

空の DB で始めるなら不要です。旧 bucket backup から戻す場合は、新 bucket に `river.db` prefix ができるようにコピーします。

```bash
mkdir -p /tmp/river-backup-restore
tar -xzf backup/litestream-backup-20260622-003554.tar.gz -C /tmp/river-backup-restore

gcloud storage cp --recursive \
  /tmp/river-backup-restore/litestream-20260622-003554/duxca-litestream-sandbox/river.db \
  gs://river-duxca-prod-litestream/
```

### 7. GCP app stack

`terraform_gcp_app` は `terraform_gcp_storage` の remote state から bucket 名と service account email を読みます。

```bash
terraform -chdir=terraform_gcp_app init -reconfigure
terraform -chdir=terraform_gcp_app plan \
  -var-file=prod.tfvars \
  -var="container_image=${CONTAINER_IMAGE}"

terraform -chdir=terraform_gcp_app apply \
  -var-file=prod.tfvars \
  -var="container_image=${CONTAINER_IMAGE}"
```

### 8. Cloudflare DNS

Cloud Run domain mapping 作成後に DNS を作ります。

ローカルでは `cf` CLI の OAuth token を使います。`cf auth whoami` で token を更新してから、更新後の access token を Terraform provider にそのコマンドだけ渡します。

```bash
cf auth whoami >/dev/null

CLOUDFLARE_API_TOKEN="$(awk -F ' = ' '/^access_token = / {gsub(/"/, "", $2); print $2}' ~/.cf/config.toml)" \
  terraform -chdir=terraform_cf init -reconfigure

CLOUDFLARE_API_TOKEN="$(awk -F ' = ' '/^access_token = / {gsub(/"/, "", $2); print $2}' ~/.cf/config.toml)" \
  terraform -chdir=terraform_cf plan -var-file=prod.tfvars

CLOUDFLARE_API_TOKEN="$(awk -F ' = ' '/^access_token = / {gsub(/"/, "", $2); print $2}' ~/.cf/config.toml)" \
  terraform -chdir=terraform_cf apply -var-file=prod.tfvars
```

### 9. 確認

```bash
gcloud run services describe litestream-sandbox \
  --project=river-duxca-prod \
  --region=asia-northeast1 \
  --format='value(status.url)'

gcloud run domain-mappings describe river.duxca.com \
  --project=river-duxca-prod \
  --region=asia-northeast1
```

## GitHub Actions deploy

`.github/workflows/deploy.yml` は `main` への merged PR を契機に実行されます。処理順は手動デプロイと同じです。

1. test / e2e test
2. `terraform_gcp_storage apply`
3. Docker image build / Artifact Registry push
4. Secret Manager の client secret version `1` の存在確認
5. `terraform_gcp_app apply`
6. `terraform_cf apply`

### GitHub Actions 用の GCP 認証

workflow は Workload Identity Federation で `github-action-river@river-duxca-prod.iam.gserviceaccount.com` を使います。このサービスアカウントは Terraform 管理対象のアプリ用 service account とは別です。`terraform_ci` で管理します。

```bash
terraform -chdir=terraform_ci init
terraform -chdir=terraform_ci plan -var-file=prod.tfvars
terraform -chdir=terraform_ci apply -var-file=prod.tfvars
```

手動で作る場合の等価コマンドは以下です。

```bash
PROJECT_ID="river-duxca-prod"
PROJECT_NUMBER="521139256632"
POOL_ID="githubaction"
PROVIDER_ID="github"
GITHUB_REPOSITORY="legokichi/river.duxca.com"
DEPLOYER_SA="github-action-river@${PROJECT_ID}.iam.gserviceaccount.com"

gcloud iam service-accounts create github-action-river \
  --project="${PROJECT_ID}" \
  --display-name="GitHub Actions River deployer"

gcloud iam workload-identity-pools create "${POOL_ID}" \
  --project="${PROJECT_ID}" \
  --location="global" \
  --display-name="GitHub Actions"

gcloud iam workload-identity-pools providers create-oidc "${PROVIDER_ID}" \
  --project="${PROJECT_ID}" \
  --location="global" \
  --workload-identity-pool="${POOL_ID}" \
  --display-name="GitHub" \
  --issuer-uri="https://token.actions.githubusercontent.com" \
  --attribute-mapping="google.subject=assertion.sub,attribute.repository=assertion.repository,attribute.actor=assertion.actor" \
  --attribute-condition="attribute.repository == '${GITHUB_REPOSITORY}'"

gcloud iam service-accounts add-iam-policy-binding "${DEPLOYER_SA}" \
  --project="${PROJECT_ID}" \
  --role="roles/iam.workloadIdentityUser" \
  --member="principalSet://iam.googleapis.com/projects/${PROJECT_NUMBER}/locations/global/workloadIdentityPools/${POOL_ID}/attribute.repository/${GITHUB_REPOSITORY}"
```

### GitHub Actions 用の IAM

deploy workflow は Terraform apply、Artifact Registry push、Cloud Run 更新、Cloudflare DNS 更新を行います。GCP 側では少なくとも次の権限が必要です。

```bash
for role in \
  roles/serviceusage.serviceUsageAdmin \
  roles/artifactregistry.admin \
  roles/artifactregistry.writer \
  roles/storage.admin \
  roles/secretmanager.admin \
  roles/iam.serviceAccountAdmin \
  roles/iam.serviceAccountUser \
  roles/run.admin
do
  gcloud projects add-iam-policy-binding "${PROJECT_ID}" \
    --member="serviceAccount:${DEPLOYER_SA}" \
    --role="${role}"
done

gcloud storage buckets add-iam-policy-binding gs://river-duxca-prod-terraform-state \
  --member="serviceAccount:${DEPLOYER_SA}" \
  --role="roles/storage.objectAdmin"
```

`terraform_gcp_app` は Cloud Run 実行用の `river-container@river-duxca-prod.iam.gserviceaccount.com` を使います。storage stack apply 後に作られるため、初回 storage apply 後に deployer へ service account user 権限を明示的に付ける運用でも構いません。

```bash
gcloud iam service-accounts add-iam-policy-binding \
  "river-container@${PROJECT_ID}.iam.gserviceaccount.com" \
  --project="${PROJECT_ID}" \
  --member="serviceAccount:${DEPLOYER_SA}" \
  --role="roles/iam.serviceAccountUser"
```

### GitHub Secrets

GitHub repository secret に Cloudflare token を保存します。

```text
CLOUDFLARE_API_TOKEN
```

OAuth の client secret は GitHub Secrets から自動投入しません。`terraform_gcp_storage apply` 後に、手動デプロイ手順と同じく Secret Manager の version `1` を作ってください。workflow は app apply 前に `FACEBOOK_CLIENT_SECRET` と `GITHUB_CLIENT_SECRET` の version `1` が読めることだけ確認します。

### Actions の Terraform plan

`.github/workflows/check.yml` の `terraform-plan` job は PR で次を実行します。

- `terraform_bootstrap`: validate / fmt / import-if-exists / plan
- `terraform_gcp_storage`: validate / fmt / plan
- `terraform_gcp_app`: validate / fmt / storage remote state がある場合だけ plan
- `terraform_ci`: validate / fmt / plan
- `terraform_cf`: validate / fmt / `CLOUDFLARE_API_TOKEN` がある場合だけ plan

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

### axum
- https://github.com/maxcountryman/axum-login
- https://github.com/maxcountryman/tower-sessions
- https://github.com/tokio-rs/axum

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





051178

川のyamap



url を共有する機能

競合がいないのはなぜ？
海外ではどう？

市場規模は？
どうやって調べる？店屋の数？売り切れ状況？店舗に行って聞く？
浅草の店に行く？
参入障壁は？
自分の足で調べないといけない？
他の川下りストはgps情報をどうやって収集してる？


市場規模は？
過去のカヌーワールドのバックナンバーを漁れば情報出そう
まずは川情報メモアプリが必要

スクレイピングをVLMでできるんじゃね？



欲しい情報は？
上流の天気
降水量
過去の事故のポイント
川の水位
瀬の場所
危険ポイント
車で乗り入れできる場所
駐車場
駅
タクシー、バスの時刻、駅の時刻
キャンプ地
入出艇禁止の場所


マネタイズどうする？
リバーツーリング業者にお金を払って情報を載せる
リバーツーリング業者にお金をもらって会社情報を載せる



最初に載せる川はどこ？関東？
カヌー人口が少ないので人は雇えない増やせない



パックラフトブームはいつ始まった？
ファルトボートはいつ終わった？


渓流釣りや沢登り、沢下りの情報も載せられない？


探検部に探索させる？

地図にコメントを書ける機能
サル発見
キャンプ地
買い出し
発着所釣りポイント
沈
瀬
橋
コース左右 
浅瀬

トイレ
水道
駐車場
温泉
駅
瀞場
買い出し場
ラーメンや
消波ブロック
障害物
コンビニ
アンダーカットロック
滝
景観
ダム
発電所
流れ
隠れ岩
水門
スタート地点
ヤナ
護岸
ビギナー向け
上級者向け

IC
堰
河原
杭
公園
川幅広いかという
交番
警察署
消防署
緊急連絡先
カヌー屋
河川局
宅急便
エスケープ
タクシー
落ち込み
レベル
ポーテージ
寄生虫
メンバースキル
レベル
水量
情報の日付
水深
釣り人
？ま
カヌーポート
川の駅

航行可能レベル
シーカヤック
SUP
カナディアンカヌー
ファルトボート
パックラフト
ダッキー
FRP
ラフト



フリースタイル・スポットプレイ
リバーランニング




北海道
石狩川
剣淵川（天塩
空知川（石狩cw25 cl6
千歳川cw17 cl6
十勝川cl6
阿寒川
風蓮川
標津川
登呂川
沙流川
余市川cl6
鵡川
釧路川cw17cw21 3cl6
尻別川cl6
美々川（ウトナイ湖cw17 cl6
網走川(cw23
湧別川(cw23
忠別川
沙流川
歴舟川忠類川 cl56
湧別川
後志利別川cl6
知内川26
天塩川 cl6
シーソラプチ川




東北（日本海
岩木川
米代川
阿仁川
雄別川
玉川（雄別
赤川
小國川（最上
最上川09 26
赤芝峡（川を下り夜に抱かれろ
寒河江川
小国横川
小国玉川
小国荒川
阿賀川

小野川湖

東北（太平洋
馬淵川
安家川
小本川
閉伊川
猿ヶ石川（北上
砂鉄川（北上
江合川「北上
北上川
阿武隈川
白石川（阿武隈
広瀬川
新川川（広瀬川
白石川（宮城県

甲信越（日本海
荒川「新潟
三面川（新潟
阿賀野川
只見川（阿賀野川
信濃川
魚野川（信濃川
千曲川（信濃川
万水川-犀川（信濃川（あずみの（安曇野5
五十嵐側（信濃川 cw15
魚野川（信濃川（cw16
黒部川
手取川（石川県や
九頭竜川（福井県
梓川
富山市松川 cl4
神通川


八ツ場あがつま湖27

野尻湖

関東（太平洋
久慈川10
那珂川3 cl6 cl8
利根川
神流川（利根川
渡良瀬川（とね
鬼怒川（とね（cw17
小貝川（とね(cw17
江戸川（とね分岐
荒川
長瀞（荒川 5 cl5
多摩川
玉川
相模川
京浜運河cw17
大横川cw17を墨田区
青野川みなみいず


中部（太平洋
笛吹川（富士川
富士川
狩野川(cw11
柿田川（狩野川
大井川08
接阻湖大井川cl5
天竜川
気田川（天竜川3
豊川（三河湾
矢作川
巴川（矢作川
木曽川
長良川
益田-飛騨川（V


4
古座川06
日置川26
吉野-紀ノ川 cw15 7
有田川
日高川5
?
中国（日本海
由良川(cw22 cw11
円山川
千代川
日野川
江ノ川 
高津川
萩・橋本川(cw25

中国（瀬戸内海
長明寺川（琵琶湖
瀬田川（琵琶湖
保津川（淀川
木津川（淀川4
瀬田-宇治川（淀川
武庫川
吉井川
鳩川
高梁川
太田川
錦川cl7
安雲川
奈良吉野川
四村川
篠山川

四国
吉野川(cw17'
那賀川
奈半利川
仁淀川(cw17 cl6
四万十川3 cl1
？原川（四万十川
肱川
海部川(徳島cw25
北川





九州
三隈-筑後川
菊池川
球磨川(cw14
球磨川（川辺川
川内川
耳川
五ケ瀬川
大野川
小川 cw28
山国川
大野川



カヌーツーリングブックに詳細あり




屈斜路湖
支笏湖
裏磐梯山湖
中禅寺湖
ならまた湖
奥利根湖
芦ノ湖
亀山湖
白丸湖
富士五湖
接阻湖
琵琶湖
音水湖
美愁湖
カヌーワールド16


朱鞠内湖
支笏湖
九頭竜湖(cw17

網走湖
四万湖
亀山湖cw23

チミケップ湖
cw24


竹生島
潮来cw12


小川原湖
cw14

7
亀山湖


接そ湖（大井川
9

5
ならまた湖


木崎湖

川の一覧

川の名前
何級河川
何水系
上流の川の一覧
河口（流入口）の位置
航行可能区間の一覧

航行可能区間の一覧
何水系
開始位置
終了位置
障害物リスト
電車の駅






道具リスト
https://thetrailsmag.com/archives/9259
https://web.goout.jp/gear/37246/
https://slackline.jp/packraft/camp-item/
https://slackline.jp/packraft/paddle/
https://slackline.jp/packraft/lets-packraft/
ドライスーツ
ウェットスーツ
ウォーターブーツ
サンダル
PFD
背負子
ナイフ
スローバッグ
ヘルメット
ポータブルポンプ
カラビナ
ハンドパドル


6/1
進水式
サイドにガムテで爪引っかからないように
PFD浮力足らない
ヘルメット要る
防水バッグ要る
雑策たくさんいる
テンションコードたくさんいる
空気抜くときブロワー使える
カラビナももっと欲しい


浸水式

軍手
プラティパス



川下り候補

東京都 多摩川 御嶽駅→青海駅
https://www.star-corp.co.jp/report/パックラフトで川を下ろう！-多摩川【御嶽駅〜青
http://yukonkawai.com/blog-entry-735.html
青梅下流
https://canoe-map.com/ken/tokyo/oume-2/
https://canoe-map.com/ken/tokyo/御岳（多摩川）カヌーツーリングマップ%e3%80%80放水口/
https://thetrailsmag.com/archives/22917

多摩川 白丸駅
https://canoe-map.com/ken/tokyo/shiromaru/



山梨県 本栖湖
http://yukonkawai.com/blog-entry-119.html

静岡県 富士川 見延線 十島駅→
https://canoe-map.com/ken/shizuoka/fujigawa/
https://canoe-map.com/ken/fujigawa-2/


茨城県 久慈川 常陸大子駅→常陸大宮
https://canoe-map.com/ken/ibaraki/kujigawa/

栃木県 那珂川
https://canoe-map.com/ken/tochigi/nakagawa-2/
https://thetrailsmag.com/archives/9186/

栃木県 鬼怒川
https://canoe-map.com/ken/tochigi-ken/kinugawa/
