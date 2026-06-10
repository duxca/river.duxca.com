#!/bin/bash
set -euo pipefail

cd "$(dirname "$0")/.."

exec ./cli/dev-local.sh
