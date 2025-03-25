#!/bin/bash

set -euxo pipefail

rm -f ./river.db

export GOOGLE_APPLICATION_CREDENTIALS=./key.json
./litestream restore -if-replica-exists -config ./litestream.yml ./river.db
./litestream replicate -exec ./server -config ./litestream.yml

