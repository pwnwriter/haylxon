#!/usr/bin/env bash
# Package a built binary into a release archive with checksum.
#
# Usage: package.sh <version> <target>
# Expects the binary at target/<target>/release/hxn[.exe]
#
# Produces:
#   haylxon-<version>-<target>.tar.gz + .sha512  (unix)
#   haylxon-<version>-<target>.zip    + .sha512  (windows)
set -euo pipefail

VERSION="$1"
TARGET="$2"
NAME="haylxon-${VERSION}"
ARCHIVE_BASE="${NAME}-${TARGET}"

# Collect release files
mkdir -p "$NAME"
cp LICENSE README.md "$NAME/"

if [[ "$TARGET" == *windows* ]]; then
    cp "target/${TARGET}/release/hxn.exe" "$NAME/"
    7z a -tzip "${ARCHIVE_BASE}.zip" "$NAME"
    shasum -a 512 "${ARCHIVE_BASE}.zip" > "${ARCHIVE_BASE}.zip.sha512"
else
    cp "target/${TARGET}/release/hxn" "$NAME/"
    tar -czvf "${ARCHIVE_BASE}.tar.gz" "$NAME/"
    shasum -a 512 "${ARCHIVE_BASE}.tar.gz" > "${ARCHIVE_BASE}.tar.gz.sha512"
fi

rm -rf "$NAME"
