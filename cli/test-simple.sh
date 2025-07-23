#!/bin/bash

set -e

# Change to project root directory
cd "$(dirname "$0")/.."

echo "Building browser with test-mode..."
cd browser
trunk build --features=test-mode --release

echo "Starting server..."
cd ../server
RUST_LOG=info cargo run --features=local &
SERVER_PID=$!

echo "Waiting for server to start..."
sleep 10

echo "Testing server is responding..."
curl -s http://localhost:8080/ | head -20

echo "Server is running, you can now run Playwright tests manually:"
echo "cd tests/e2e && npx playwright test"

echo "To stop server: kill $SERVER_PID"