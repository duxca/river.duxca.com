#!/bin/bash

set -euxvo pipefail

#gcloud storage rm --recursive gs://duxca-litestream-sandbox/river.db || true

pushd db
sqlx database reset -y
export GOOGLE_APPLICATION_CREDENTIALS=./key.json
./litestream replicate -config ./litestream.yml -exec "sleep 5"
popd
