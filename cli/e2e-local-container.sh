#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

SERVER_PORT="${SERVER_PORT:-18080}"
E2E_IMAGE="${E2E_IMAGE:-river-duxca-e2e:local}"
E2E_WAIT_ATTEMPTS="${E2E_WAIT_ATTEMPTS:-1800}"
E2E_START_SERVER="${E2E_START_SERVER:-1}"

if [[ "${1:-}" == "--no-server" ]]; then
  E2E_START_SERVER=0
  shift
fi

if [[ $# -ne 0 ]]; then
  echo "usage: $0 [--no-server]" >&2
  exit 2
fi

cleanup() {
  if [[ -n "${DEV_PID:-}" ]]; then
    kill "$DEV_PID" 2>/dev/null || true
    wait "$DEV_PID" 2>/dev/null || true
  fi
}
trap cleanup EXIT INT TERM

docker build -f e2e/Dockerfile -t "$E2E_IMAGE" .

if [[ "$E2E_START_SERVER" != "0" ]]; then
  SERVER_HOST=0.0.0.0 ./cli/dev-local.sh &
  DEV_PID=$!
fi

for _ in $(seq 1 "$E2E_WAIT_ATTEMPTS"); do
  if curl -fsS "http://127.0.0.1:${SERVER_PORT}/app" >/dev/null 2>&1; then
    break
  fi
  sleep 0.5
done

if ! curl -fsS "http://127.0.0.1:${SERVER_PORT}/app" >/dev/null 2>&1; then
  echo "frontend did not start on http://127.0.0.1:${SERVER_PORT}/app" >&2
  exit 1
fi

docker run --rm \
  --add-host=host.docker.internal:host-gateway \
  -e SERVER_URL="http://host.docker.internal:${SERVER_PORT}" \
  -e FRONTEND_URL="http://host.docker.internal:${SERVER_PORT}/app" \
  "$E2E_IMAGE"
