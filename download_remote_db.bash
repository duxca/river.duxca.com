#!/bin/bash

set -euxvo pipefail



pushd ./db
rm -f river.db
export GOOGLE_APPLICATION_CREDENTIALS=./key.json
./litestream restore -if-replica-exists -config litestream.yml river.db
popd
