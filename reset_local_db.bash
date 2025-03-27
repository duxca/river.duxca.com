#!/bin/bash

set -euxvo pipefail

pushd db
sqlx database reset -y
popd
