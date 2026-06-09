#!/usr/bin/env bash
# Upload release archives to a GitHub release.
#
# Usage: upload.sh <tag> <repo> <version> <target>
# Expects archives produced by package.sh in the current directory.
set -euo pipefail

TAG="$1"
REPO="$2"
VERSION="$3"
TARGET="$4"
ARCHIVE_BASE="haylxon-${VERSION}-${TARGET}"

# Create the release if it doesn't exist yet (other matrix jobs may race here)
gh release create "$TAG" --repo "$REPO" --title "$TAG" --notes "" 2>/dev/null || true

# Upload all matching files (archive + checksum)
for file in "${ARCHIVE_BASE}".*; do
    gh release upload "$TAG" "$file" --repo "$REPO" --clobber
done
