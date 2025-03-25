#!/bin/bash

set -euxo pipefail

pushd db
sqlx database reset -y
popd
