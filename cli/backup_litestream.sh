#!/bin/bash
set -euo pipefail

# Litestream GCS Backup Script
# This script creates a complete backup of the Litestream GCS bucket

BUCKET_NAME="duxca-litestream-sandbox"
# Change to project root directory
cd "$(dirname "$0")/.."

BACKUP_BASE_DIR="$(pwd)/backup"
TIMESTAMP=$(date +%Y%m%d-%H%M%S)
BACKUP_DIR="${BACKUP_BASE_DIR}/litestream-${TIMESTAMP}"

echo "Starting Litestream backup at $(date)"
echo "Backup directory: ${BACKUP_DIR}"

# Create backup directory
mkdir -p "${BACKUP_DIR}"

# Backup bucket contents
echo "Backing up bucket contents..."
gsutil -m cp -r "gs://${BUCKET_NAME}" "${BACKUP_DIR}/"

# Backup bucket metadata and configuration
echo "Backing up bucket metadata..."
gsutil ls -L -b "gs://${BUCKET_NAME}" > "${BACKUP_DIR}/bucket_metadata.txt"

# List all files with details
echo "Creating file inventory..."
gsutil ls -r -l "gs://${BUCKET_NAME}" > "${BACKUP_DIR}/file_inventory.txt"

# Create a backup info file
cat > "${BACKUP_DIR}/backup_info.txt" << EOF
Litestream Backup Information
============================
Backup Date: $(date)
Bucket Name: ${BUCKET_NAME}
Backup Directory: ${BACKUP_DIR}
Backup Size: $(du -sh "${BACKUP_DIR}" | cut -f1)

Files backed up:
$(find "${BACKUP_DIR}/${BUCKET_NAME}" -type f | wc -l) files

Backup Contents:
$(tree "${BACKUP_DIR}/${BUCKET_NAME}" 2>/dev/null || find "${BACKUP_DIR}/${BUCKET_NAME}" -type f)
EOF

# Create a compressed archive
echo "Creating compressed archive..."
cd "${BACKUP_BASE_DIR}"
tar -czf "litestream-backup-${TIMESTAMP}.tar.gz" "litestream-${TIMESTAMP}/"

echo "Backup completed successfully!"
echo "Backup location: ${BACKUP_DIR}"
echo "Compressed archive: ${BACKUP_BASE_DIR}/litestream-backup-${TIMESTAMP}.tar.gz"
echo "Backup size: $(du -sh "${BACKUP_DIR}" | cut -f1)"
echo "Archive size: $(du -sh "${BACKUP_BASE_DIR}/litestream-backup-${TIMESTAMP}.tar.gz" | cut -f1)"