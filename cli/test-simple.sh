#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

exec ./cli/e2e-local-container.sh
