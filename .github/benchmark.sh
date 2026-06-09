#!/usr/bin/env bash
set -euo pipefail

CHROME_BIN="${CHROME_BIN:-/usr/bin/chrome}"
URL_FILE="${URL_FILE:-/tmp/bench_urls.txt}"
RUNS="${RUNS:-3}"
WARMUP="${WARMUP:-1}"

# Create test URLs if file doesn't exist
if [ ! -f "$URL_FILE" ]; then
    cat > "$URL_FILE" << 'EOF'
https://example.com
https://httpbin.org
https://www.google.com
https://github.com
https://wikipedia.org
EOF
fi

cleanup() {
    rm -rf hxnshots screenshots gowitness.sqlite3
}

echo "=== hxn vs gowitness benchmark ==="
echo "Chrome: $CHROME_BIN"
echo "URLs:   $URL_FILE ($(wc -l < "$URL_FILE") URLs)"
echo "Runs:   $RUNS | Warmup: $WARMUP"
echo ""

echo "--- Single URL ---"
hyperfine \
    --warmup "$WARMUP" \
    --runs "$RUNS" \
    --cleanup 'rm -rf hxnshots screenshots gowitness.sqlite3' \
    --command-name "hxn" \
    "hxn -u https://example.com -b $CHROME_BIN -s" \
    --command-name "gowitness" \
    "gowitness scan single --url https://example.com --write-db=false"
cleanup

echo ""
echo "--- Bulk ($(wc -l < "$URL_FILE") URLs, 4 tabs) ---"
# gowitness reads from stdin for bulk
hyperfine \
    --warmup "$WARMUP" \
    --runs "$RUNS" \
    --cleanup 'rm -rf hxnshots screenshots gowitness.sqlite3' \
    --command-name "hxn (4 tabs)" \
    "hxn -f $URL_FILE -b $CHROME_BIN -s" \
    --command-name "gowitness (4 threads)" \
    "cat $URL_FILE | gowitness scan file -f - --threads 4 --write-db=false"
cleanup

echo ""
echo "--- Bulk ($(wc -l < "$URL_FILE") URLs, 8 tabs) ---"
hyperfine \
    --warmup "$WARMUP" \
    --runs "$RUNS" \
    --cleanup 'rm -rf hxnshots screenshots gowitness.sqlite3' \
    --command-name "hxn (8 tabs)" \
    "hxn -f $URL_FILE -b $CHROME_BIN -s -t 8" \
    --command-name "gowitness (8 threads)" \
    "cat $URL_FILE | gowitness scan file -f - --threads 8 --write-db=false"
cleanup
