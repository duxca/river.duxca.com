#!/bin/bash

set -euxvo pipefail

# Change to project root directory
cd "$(dirname "$0")/.."

pushd ./db
rm -f river.db
export LITESTREAM_BUCKET="${LITESTREAM_BUCKET:-duxca-litestream-sandbox}"
export GOOGLE_APPLICATION_CREDENTIALS=./key.json
./litestream restore -if-replica-exists -config litestream.yml river.db
popd
