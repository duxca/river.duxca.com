# E2E Tests for River.duxca.com

End-to-end tests using Playwright to test user journeys and functionality.

## Setup

1. Install dependencies:
   ```bash
   npm install
   ```

2. Install Playwright browsers:
   ```bash
   npx playwright install
   ```

## Running Tests

### Local Development

Run all tests with the test script:
```bash
./run-tests.sh
```

Or run manually:
```bash
# Build browser with test-mode feature
cd ../../browser
trunk build --features=test-mode --release

# Run tests
cd ../tests/e2e
npm test
```

### Test Modes

- `npm test` - Run all tests headless
- `npm run test:headed` - Run tests with browser visible
- `npm run test:debug` - Run tests in debug mode

## Test Structure

- `tests/basic.spec.ts` - Basic navigation and authentication tests
- `tests/river-management.spec.ts` - River, waypoint, and track management tests
- `tests/map-interaction.spec.ts` - Map functionality tests

## Test Mode Feature

The tests use a `test-mode` feature flag that:
- Bypasses OAuth authentication
- Automatically logs in with a test user
- Allows testing without external dependencies

## Configuration

- `playwright.config.ts` - Main Playwright configuration
- `package.json` - Node.js dependencies and scripts
- `run-tests.sh` - Test execution script

## CI/CD

Tests run automatically on:
- Push to main branch
- Pull requests to main branch

See `.github/workflows/e2e-tests.yml` for CI configuration.