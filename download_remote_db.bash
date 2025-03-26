#!/bin/bash

set -euxo pipefail



pushd ./db
rm -f river.db
export GOOGLE_APPLICATION_CREDENTIALS=../key.json
../assets/litestream restore -if-replica-exists -config litestream.yml river.db
popd
