#!/bin/bash

# Update Claude credentials script
# Updates GitHub repository secrets with current Claude OAuth credentials

set -euo pipefail

CREDENTIALS_FILE="$HOME/.claude/.credentials.json"

# Check if credentials file exists
if [[ ! -f "$CREDENTIALS_FILE" ]]; then
    echo "❌ Error: Claude credentials file not found at $CREDENTIALS_FILE"
    echo "Please make sure you're logged in to Claude Code CLI"
    exit 1
fi

# Check if gh CLI is installed and authenticated
if ! command -v gh &> /dev/null; then
    echo "❌ Error: GitHub CLI (gh) is not installed"
    echo "Please install gh CLI: https://cli.github.com/"
    exit 1
fi

if ! gh auth status &> /dev/null; then
    echo "❌ Error: GitHub CLI is not authenticated"
    echo "Please run: gh auth login"
    exit 1
fi

echo "🔍 Reading Claude credentials from $CREDENTIALS_FILE"

# Extract credentials using jq
if ! command -v jq &> /dev/null; then
    echo "❌ Error: jq is not installed"
    echo "Please install jq: sudo apt-get install jq"
    exit 1
fi

ACCESS_TOKEN=$(jq -r '.claudeAiOauth.accessToken' "$CREDENTIALS_FILE")
REFRESH_TOKEN=$(jq -r '.claudeAiOauth.refreshToken' "$CREDENTIALS_FILE")
EXPIRES_AT=$(jq -r '.claudeAiOauth.expiresAt' "$CREDENTIALS_FILE")

# Validate extracted values
if [[ "$ACCESS_TOKEN" == "null" || "$REFRESH_TOKEN" == "null" || "$EXPIRES_AT" == "null" ]]; then
    echo "❌ Error: Failed to extract credentials from $CREDENTIALS_FILE"
    echo "Please check the file format"
    exit 1
fi

echo "📅 Token expires at: $(date -d @$((EXPIRES_AT / 1000)) '+%Y-%m-%d %H:%M:%S')"

# Check if token is expired
CURRENT_TIME=$(date +%s)
EXPIRES_TIME=$((EXPIRES_AT / 1000))

if [[ $CURRENT_TIME -gt $EXPIRES_TIME ]]; then
    echo "⚠️  Warning: Token appears to be expired"
    echo "You may need to refresh your Claude Code CLI login"
fi

echo "🔄 Updating GitHub repository secrets..."

# Update secrets
gh secret set CLAUDE_ACCESS_TOKEN --body "$ACCESS_TOKEN"
gh secret set CLAUDE_REFRESH_TOKEN --body "$REFRESH_TOKEN"  
gh secret set CLAUDE_EXPIRES_AT --body "$EXPIRES_AT"

echo "✅ Successfully updated Claude credentials in GitHub repository secrets"
echo "🚀 Claude Actions should now work with updated credentials"

# List current secrets to verify
echo ""
echo "📋 Current repository secrets:"
gh secret list | grep -E "(CLAUDE_|BRAVE_)"