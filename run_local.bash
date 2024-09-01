#!/bin/bash
set -euxo pipefail
pushd server
sqlx database reset -y
popd
{ cd server && cargo watch -x "run --features=local"; } & { cd browser && trunk watch; }