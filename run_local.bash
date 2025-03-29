#!/bin/bash
set -euxvo pipefail

{
    cd server;
    cargo watch -x "run --features=local";
} & {
    cd browser;
    trunk watch --features=local;
}

trap 'kill $(jobs -p) 2>/dev/null' EXIT
