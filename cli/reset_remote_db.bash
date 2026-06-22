#!/bin/bash

set -euxvo pipefail

# Change to project root directory
cd "$(dirname "$0")/.."

#gcloud storage rm --recursive "gs://${LITESTREAM_BUCKET:-duxca-litestream-sandbox}/river.db" || true

pushd db
sqlx database reset -y
export LITESTREAM_BUCKET="${LITESTREAM_BUCKET:-duxca-litestream-sandbox}"
export GOOGLE_APPLICATION_CREDENTIALS=./key.json
./litestream replicate -config ./litestream.yml -exec "sleep 5"
popd
