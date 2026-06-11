#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

SERVER_HOST="${SERVER_HOST:-127.0.0.1}"
SERVER_PORT="${SERVER_PORT:-18080}"
LISTEN_ADDR="${LEPTOS_SITE_ADDR:-${HOST_ADDR:-${SERVER_HOST}:${SERVER_PORT}}}"
SITE_ROOT="${LEPTOS_SITE_ROOT:-${LOCAL_DIST_PATH:-target/site}}"
LOCAL_DB_DIR="${LOCAL_DB_DIR:-.local}"
LOCAL_DB_PATH="${LOCAL_DB_PATH:-${LOCAL_DB_DIR}/river-dev.db}"

mkdir -p "$LOCAL_DB_DIR"

cleanup() {
  if [[ -n "${SERVER_PID:-}" ]]; then
    kill "$SERVER_PID" 2>/dev/null || true
  fi
}
trap cleanup EXIT INT TERM

export RUST_LOG="${RUST_LOG:-server=debug,service=debug,db=debug,tower_sessions=info}"
unset HOST_ADDR
unset LOCAL_DIST_PATH
export DATABASE_URL="${DATABASE_URL:-sqlite://${LOCAL_DB_PATH}?mode=rwc}"
export BASE_URL="${BASE_URL:-http://${SERVER_HOST}:${SERVER_PORT}}"
export LOCAL_BASE_URL="${LOCAL_BASE_URL:-http://${SERVER_HOST}:${SERVER_PORT}}"
export LOCAL_CLIENT_ID="${LOCAL_CLIENT_ID:-local}"
export LOCAL_CLIENT_SECRET="${LOCAL_CLIENT_SECRET:-local}"
export GITHUB_CLIENT_ID="${GITHUB_CLIENT_ID:-dummy}"
export GITHUB_CLIENT_SECRET="${GITHUB_CLIENT_SECRET:-dummy}"
export FACEBOOK_CLIENT_ID="${FACEBOOK_CLIENT_ID:-dummy}"
export FACEBOOK_CLIENT_SECRET="${FACEBOOK_CLIENT_SECRET:-dummy}"
export LEPTOS_OUTPUT_NAME="${LEPTOS_OUTPUT_NAME:-leptos-browser}"
export LEPTOS_SITE_ROOT="${SITE_ROOT}"
export LEPTOS_SITE_PKG_DIR="${LEPTOS_SITE_PKG_DIR:-pkg}"
export LEPTOS_SITE_ADDR="${LISTEN_ADDR}"
export LEPTOS_RELOAD_PORT="${LEPTOS_RELOAD_PORT:-18082}"

if ! cargo leptos --help >/dev/null 2>&1; then
  echo "cargo-leptos is required. Install with: cargo binstall -y cargo-leptos@0.3.6" >&2
  exit 1
fi

cargo sqlx database setup --source db/migrations --no-dotenv --database-url "$DATABASE_URL"

echo "server:   http://${SERVER_HOST}:${SERVER_PORT}/"
echo "frontend: http://${SERVER_HOST}:${SERVER_PORT}/app"

exec cargo leptos watch
