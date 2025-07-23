#!/bin/bash
set -euxvo pipefail

rm -f ./river.db

env | sort
ls -la /etc
cat .env | sort
cat ./litestream.yml
echo $GOOGLE_APPLICATION_CREDENTIALS > ./key.json
export GOOGLE_APPLICATION_CREDENTIALS=./key.json
ls -la .
cat $GOOGLE_APPLICATION_CREDENTIALS
./litestream restore -config ./litestream.yml -if-replica-exists ./river.db
./litestream replicate -config ./litestream.yml -exec ./server

