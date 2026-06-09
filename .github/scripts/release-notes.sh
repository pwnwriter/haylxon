#!/usr/bin/env bash
# Generate release notes with a download table and update the GitHub release.
#
# Usage: release-notes.sh <tag> <repo>
set -euo pipefail

TAG="$1"
REPO="$2"
VERSION="${TAG#v}"

ASSETS=$(gh release view "$TAG" --repo "$REPO" --json assets -q '.assets[].name')

BODY="## Installation

\`\`\`sh
cargo install hxn --locked
\`\`\`

Or download a prebuilt binary below.

## Download

| Target | Binary | SHA-512 |
|--------|--------|---------|"

UNIX_TARGETS=(
    x86_64-unknown-linux-gnu
    x86_64-unknown-linux-musl
    i686-unknown-linux-gnu
    i686-unknown-linux-musl
    armv5te-unknown-linux-gnueabi
    armv7-unknown-linux-gnueabihf
    aarch64-unknown-linux-gnu
    aarch64-unknown-linux-musl
    x86_64-apple-darwin
    aarch64-apple-darwin
)

WIN_TARGETS=(
    x86_64-pc-windows-msvc
    i686-pc-windows-msvc
)

add_row() {
    local file="$1" target="$2"
    local url="https://github.com/${REPO}/releases/download/${TAG}/${file}"
    local sha

    if echo "$ASSETS" | grep -q "^${file}$"; then
        sha=$(gh release download "$TAG" --repo "$REPO" --pattern "$file" --output - \
            | shasum -a 512 | cut -d' ' -f1)
        BODY="${BODY}
| \`${target}\` | [${file}](${url}) | \`${sha:0:16}...\` |"
    fi
}

for target in "${UNIX_TARGETS[@]}"; do
    add_row "haylxon-${VERSION}-${target}.tar.gz" "$target"
done

for target in "${WIN_TARGETS[@]}"; do
    add_row "haylxon-${VERSION}-${target}.zip" "$target"
done

gh release edit "$TAG" --repo "$REPO" --notes "$BODY"
