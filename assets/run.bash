#!/bin/bash

set -euxo pipefail

rm -f ./river.db

./litestream restore -if-replica-exists -config ./litestream.yml ./river.db
./litestream replicate -exec ./server -config ./litestream.yml

