#!/bin/bash
set -euxvo pipefail

rm -f ./river.db

env | sort
cat $GOOGLE_APPLICATION_CREDENTIALS
cat .env | sort
cat ./litestream.yml
export GOOGLE_APPLICATION_CREDENTIALS=./key.json
cat $GOOGLE_APPLICATION_CREDENTIALS
./litestream restore -config ./litestream.yml -if-replica-exists ./river.db
./litestream replicate -config ./litestream.yml -exec ./server

