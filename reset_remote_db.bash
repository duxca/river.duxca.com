#!/bin/bash

set -euxo pipefail

gcloud storage rm --recursive gs://duxca-litestream-sandbox/river.db || true

pushd db
sqlx database reset -y
popd

export GOOGLE_APPLICATION_CREDENTIALS=./key.json
./litestream replicate -config litestream.yml -exec "sleep 5"

