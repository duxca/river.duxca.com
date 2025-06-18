#!/bin/bash

# Script to run E2E tests with test mode enabled

set -e

echo "Building browser with test-mode feature..."
cd ../../browser
trunk build --features=test-mode --release

echo "Installing E2E test dependencies..."
cd ../tests/e2e
npm install

echo "Installing Playwright browsers..."
npx playwright install

echo "Running E2E tests..."
npm test