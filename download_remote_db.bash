#!/bin/bash

set -euxo pipefail

rm -f ./db/river.db

export GOOGLE_APPLICATION_CREDENTIALS=./key.json
./litestream restore -if-replica-exists -config ./litestream.yml ./db/river.db

