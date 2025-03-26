#!/bin/bash

set -euxo pipefail

pushd db
export GOOGLE_APPLICATION_CREDENTIALS=../key.json
../assets/litestream replicate -config litestream.yml -exec "sleep 5"
popd
