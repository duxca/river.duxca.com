#!/bin/bash
set -euo pipefail

# Litestream GCS Restore Script
# This script restores a Litestream backup to GCS bucket

BUCKET_NAME="${LITESTREAM_BUCKET:-duxca-litestream-sandbox}"
# Change to project root directory
cd "$(dirname "$0")/.."

BACKUP_BASE_DIR="$(pwd)/backup"

# Function to show usage
usage() {
    echo "Usage: $0 [backup_timestamp|backup_archive.tar.gz]"
    echo ""
    echo "Examples:"
    echo "  $0 20250606-064723                              # Restore from backup directory"
    echo "  $0 litestream-backup-20250606-064723.tar.gz     # Restore from compressed archive"
    echo "  $0                                               # Show available backups"
    echo ""
    exit 1
}

# Function to list available backups
list_backups() {
    echo "Available backups:"
    echo "=================="
    
    # List backup directories
    if ls "${BACKUP_BASE_DIR}"/litestream-* >/dev/null 2>&1; then
        echo "Backup directories:"
        for dir in "${BACKUP_BASE_DIR}"/litestream-*; do
            if [ -d "$dir" ]; then
                timestamp=$(basename "$dir" | sed 's/litestream-//')
                size=$(du -sh "$dir" | cut -f1)
                echo "  $timestamp (${size})"
            fi
        done
        echo ""
    fi
    
    # List backup archives
    if ls "${BACKUP_BASE_DIR}"/litestream-backup-*.tar.gz >/dev/null 2>&1; then
        echo "Backup archives:"
        for archive in "${BACKUP_BASE_DIR}"/litestream-backup-*.tar.gz; do
            if [ -f "$archive" ]; then
                timestamp=$(basename "$archive" | sed 's/litestream-backup-//' | sed 's/.tar.gz//')
                size=$(du -sh "$archive" | cut -f1)
                echo "  $timestamp (${size})"
            fi
        done
    fi
}

# Check if backup parameter is provided
if [ $# -eq 0 ]; then
    list_backups
    usage
fi

BACKUP_INPUT="$1"

# Determine if input is a timestamp or archive file
if [[ "$BACKUP_INPUT" == *.tar.gz ]]; then
    # Input is an archive file
    ARCHIVE_PATH="${BACKUP_BASE_DIR}/${BACKUP_INPUT}"
    if [ ! -f "$ARCHIVE_PATH" ]; then
        echo "Error: Archive file not found: $ARCHIVE_PATH"
        exit 1
    fi
    
    echo "Extracting archive: $ARCHIVE_PATH"
    cd "$BACKUP_BASE_DIR"
    tar -xzf "$ARCHIVE_PATH"
    
    # Extract timestamp from archive name
    TIMESTAMP=$(echo "$BACKUP_INPUT" | sed 's/litestream-backup-//' | sed 's/.tar.gz//')
    BACKUP_DIR="${BACKUP_BASE_DIR}/litestream-${TIMESTAMP}"
else
    # Input is a timestamp
    TIMESTAMP="$BACKUP_INPUT"
    BACKUP_DIR="${BACKUP_BASE_DIR}/litestream-${TIMESTAMP}"
fi

# Check if backup directory exists
if [ ! -d "$BACKUP_DIR" ]; then
    echo "Error: Backup directory not found: $BACKUP_DIR"
    echo ""
    list_backups
    exit 1
fi

# Check if bucket data exists in backup
BUCKET_BACKUP_DIR="${BACKUP_DIR}/${BUCKET_NAME}"
if [ ! -d "$BUCKET_BACKUP_DIR" ]; then
    echo "Error: Bucket data not found in backup: $BUCKET_BACKUP_DIR"
    exit 1
fi

echo "Starting Litestream restore from backup: $TIMESTAMP"
echo "Backup directory: $BACKUP_DIR"
echo "Target bucket: gs://$BUCKET_NAME"

# Show backup info if available
if [ -f "${BACKUP_DIR}/backup_info.txt" ]; then
    echo ""
    echo "Backup Information:"
    echo "==================="
    cat "${BACKUP_DIR}/backup_info.txt"
    echo ""
fi

# Confirm with user
read -p "Are you sure you want to restore this backup to gs://${BUCKET_NAME}? This will overwrite existing data. (y/N): " -r
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Restore cancelled."
    exit 0
fi

# Backup current bucket state before restore
CURRENT_BACKUP_DIR="${BACKUP_BASE_DIR}/pre-restore-backup-$(date +%Y%m%d-%H%M%S)"
echo "Creating backup of current bucket state: $CURRENT_BACKUP_DIR"
mkdir -p "$CURRENT_BACKUP_DIR"
gsutil -m cp -r "gs://${BUCKET_NAME}" "$CURRENT_BACKUP_DIR/" || echo "Warning: Could not backup current state (bucket might be empty)"

# Clear existing bucket contents
echo "Clearing existing bucket contents..."
gsutil -m rm -r "gs://${BUCKET_NAME}/**" || echo "Bucket was empty or already cleared"

# Restore from backup
echo "Restoring files from backup..."
gsutil -m cp -r "${BUCKET_BACKUP_DIR}/*" "gs://${BUCKET_NAME}/"

# Verify restore
echo "Verifying restore..."
echo "Files in bucket after restore:"
gsutil ls -r "gs://${BUCKET_NAME}"

echo ""
echo "Restore completed successfully!"
echo "Restored from: $BACKUP_DIR"
echo "Files restored to: gs://${BUCKET_NAME}"
echo "Pre-restore backup saved to: $CURRENT_BACKUP_DIR"
