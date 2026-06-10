#!/bin/bash
set -euo pipefail

rm -f ./river.db

if [[ -n "${KEY_JSON:-}" ]]; then
  printf '%s' "$KEY_JSON" > ./key.json
  export GOOGLE_APPLICATION_CREDENTIALS=./key.json
elif [[ -n "${GOOGLE_APPLICATION_CREDENTIALS:-}" && "${GOOGLE_APPLICATION_CREDENTIALS:0:1}" == "{" ]]; then
  printf '%s' "$GOOGLE_APPLICATION_CREDENTIALS" > ./key.json
  export GOOGLE_APPLICATION_CREDENTIALS=./key.json
fi

./litestream restore -config ./litestream.yml -if-replica-exists ./river.db
exec ./litestream replicate -config ./litestream.yml -exec ./server
