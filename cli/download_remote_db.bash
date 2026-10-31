#!/bin/bash

set -euxvo pipefail

# Change to project root directory
cd "$(dirname "$0")/.."

pushd ./db
LITESTREAM="${LITESTREAM:-../.local/bin/litestream}"
[ -x "$LITESTREAM" ] || { echo "run 'make litestream' first" >&2; exit 1; }
rm -f river.db
export LITESTREAM_BUCKET="${LITESTREAM_BUCKET:?LITESTREAM_BUCKET is required}"
export GOOGLE_APPLICATION_CREDENTIALS=./key.json
"$LITESTREAM" restore -if-replica-exists -config litestream.yml river.db
popd
