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
https://stackoverflow.com
https://reddit.com
https://rust-lang.org
https://crates.io
https://docs.rs
https://www.mozilla.org
https://developer.mozilla.org
https://nodejs.org
https://go.dev
https://python.org
https://www.typescriptlang.org
https://vuejs.org
https://reactjs.org
https://angular.io
https://svelte.dev
https://tailwindcss.com
https://getbootstrap.com
https://jquery.com
https://expressjs.com
https://fastapi.tiangolo.com
https://flask.palletsprojects.com
https://djangoproject.com
https://spring.io
https://laravel.com
https://rubyonrails.org
https://elixir-lang.org
https://www.haskell.org
https://kotlinlang.org
https://www.scala-lang.org
https://ziglang.org
https://vlang.io
https://nim-lang.org
https://crystal-lang.org
https://dart.dev
https://flutter.dev
https://brew.sh
https://archlinux.org
https://ubuntu.com
https://fedoraproject.org
https://www.debian.org
https://nixos.org
https://gitlab.com
https://bitbucket.org
https://sourceforge.net
https://codeberg.org
https://sr.ht
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
