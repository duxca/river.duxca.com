#!/bin/bash
set -euxvo pipefail

# Change to project root directory
cd "$(dirname "$0")/.."

{
    cd server;
    export RUST_LOG=service=trace,db=trace,server=trace,model=trace,tower_sessions=trace;
    cargo watch -x "run --features=local";
} & {
    cd browser;
    trunk watch --features=local;
}

trap 'kill $(jobs -p) 2>/dev/null' EXIT
