#!/bin/bash

set -euxvo pipefail

# Change to project root directory
cd "$(dirname "$0")/.."

pushd db
sqlx database reset -y
popd
