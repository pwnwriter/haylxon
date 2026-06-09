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
    rm -rf hxnshots gowitness-* screenshots
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
    --cleanup 'rm -rf hxnshots gowitness-* screenshots' \
    --command-name "hxn" \
    "hxn -u https://example.com -b $CHROME_BIN -s" \
    --command-name "gowitness" \
    "gowitness single https://example.com"
cleanup

echo ""
echo "--- Bulk ($(wc -l < "$URL_FILE") URLs, 4 tabs) ---"
hyperfine \
    --warmup "$WARMUP" \
    --runs "$RUNS" \
    --cleanup 'rm -rf hxnshots gowitness-* screenshots' \
    --command-name "hxn (4 tabs)" \
    "hxn -f $URL_FILE -b $CHROME_BIN -s" \
    --command-name "gowitness (4 threads)" \
    "gowitness file -f $URL_FILE --threads 4"
cleanup

echo ""
echo "--- Bulk ($(wc -l < "$URL_FILE") URLs, 8 tabs) ---"
hyperfine \
    --warmup "$WARMUP" \
    --runs "$RUNS" \
    --cleanup 'rm -rf hxnshots gowitness-* screenshots' \
    --command-name "hxn (8 tabs)" \
    "hxn -f $URL_FILE -b $CHROME_BIN -s -t 8" \
    --command-name "gowitness (8 threads)" \
    "gowitness file -f $URL_FILE --threads 8"
cleanup
