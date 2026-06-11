#!/bin/bash
set -euo pipefail

restore_dir="$(mktemp -d ./river-db-restore.XXXXXX)"
restore_db="${restore_dir}/river.db"

cleanup() {
    rm -rf "$restore_dir"
}
trap cleanup EXIT

./litestream restore -config ./litestream.yml -if-replica-exists -o "$restore_db" ./river.db

if [ -f "$restore_db" ]; then
    mv -f "$restore_db" ./river.db
fi

cleanup
trap - EXIT

exec ./litestream replicate -config ./litestream.yml -exec ./server
