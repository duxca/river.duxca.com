#!/bin/bash
set -euo pipefail

: "${LITESTREAM_BUCKET:=duxca-litestream-sandbox}"
export LITESTREAM_BUCKET

restore_dir="$(mktemp -d ./river-db-restore.XXXXXX)"
restore_db="${restore_dir}/river.db"

cleanup() {
    rm -rf "$restore_dir"
}
trap cleanup EXIT

./litestream restore -config ./litestream.yml -if-replica-exists -o "$restore_db" ./river.db

if [ -f "$restore_db" ]; then
    # restore に成功したら使うようにする
    mv -f "$restore_db" ./river.db
fi

cleanup
trap - EXIT

exec ./litestream replicate -config ./litestream.yml -exec ./server
