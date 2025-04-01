#!/bin/bash
set -euxvo pipefail

{
    cd server;
    export RUST_LOG=service=trace,db=trace,server=trace,model=trace;
    cargo watch -x "run --features=local";
} & {
    cd browser;
    trunk watch --features=local;
}

trap 'kill $(jobs -p) 2>/dev/null' EXIT
