#!/bin/bash
set -euo pipefail

rm -f ./river.db

./litestream restore -config ./litestream.yml -if-replica-exists ./river.db
exec ./litestream replicate -config ./litestream.yml -exec ./server
