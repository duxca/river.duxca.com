#!/bin/bash

set -euxo pipefail

# Change to project root directory
cd "$(dirname "$0")/.."

pushd db
export LITESTREAM_BUCKET="${LITESTREAM_BUCKET:-duxca-litestream-sandbox}"
export GOOGLE_APPLICATION_CREDENTIALS=./key.json
./litestream replicate -config ./litestream.yml -exec "sleep 5"
popd
