<img src="https://raw.githubusercontent.com/pwnwriter/haylxon/showcase/hxn-transparent.png"  width="200" align="right">

# Haylxon

**Shoot before the blink** — a fast, minimal screenshot tool for webpages using Chrome's headless mode. Supports local and remote browsers, tab pooling, and parallel execution.

[![GitHub Release](https://img.shields.io/github/v/release/pwnwriter/haylxon?style=flat&labelColor=56534b&color=c1c1b6&logo=GitHub&logoColor=white)](https://github.com/pwnwriter/haylxon/releases)
[![Crate](https://img.shields.io/crates/v/hxn?style=flat&labelColor=56534b&color=c1c1b6&logo=Rust&logoColor=white)](https://crates.io/crates/hxn/)
[![License](https://img.shields.io/badge/License-MIT-white.svg)](https://github.com/pwnwriter/haylxon/blob/main/LICENSE)
[![ko-fi](https://img.shields.io/badge/support-pwnwriter%20-pink?logo=kofi&logoColor=white)](https://ko-fi.com/pwnwriter)

---

## Features

- **Local or remote** — launch a local browser or connect to a remote Chromium instance via CDP
- **Tab pooling** — reuse browser tabs across URLs instead of spawning new ones every time
- **Parallel** — configurable concurrency with `--tabs`
- **Flexible input** — single URL, file, or stdin
- **Output formats** — PNG, JPEG, WebP, fullpage, NDJSON metadata
- **Portable** — Linux, macOS, Windows, Termux

## Installation

<details> <summary><b>Binary</b></summary>

Download from [releases](https://github.com/pwnwriter/haylxon/releases), or:

```bash
wget -qO- "$(curl -qfsSL "https://api.github.com/repos/pwnwriter/haylxon/releases/latest" \
  | jq -r '.assets[].browser_download_url' \
  | grep -Ei "$(uname -m).*$(uname -s).*musl" \
  | grep -v "\.sha")" | tar -xzf - --strip-components=1 && ./hxn -h
```
</details>

<details> <summary><b>Cargo</b></summary>

```bash
cargo install hxn
# or with binstall
cargo binstall hxn
```
</details>

<details> <summary><b>Source</b></summary>

```bash
git clone --depth=1 https://github.com/pwnwriter/haylxon
cd haylxon && cargo build --release
```
</details>

<details> <summary><b>Nix</b></summary>

```bash
nix run github:pwnwriter/haylxon
# or
nix profile install nixpkgs#haylxon
```
</details>

<details> <summary><b>Arch Linux</b></summary>

```bash
paru -S haylxon-git
```
</details>

<details> <summary><b>Termux</b></summary>

```bash
pkg install tur-repo -y && pkg install chromium -y
ln -s "$PREFIX/bin/chromium-browser" "$PREFIX/bin/chromium"
```
</details>

---

## Usage

> Examples use `brave` — any Chromium-based browser works. Swap `-b $(which brave)` with your browser path.

### Basic

```bash
# Single URL
hxn -b $(which brave) -u https://example.com

# From a file
hxn -b $(which brave) -f urls.txt

# From stdin
cat urls.txt | hxn -b $(which brave) --stdin
```

<details> <summary><b>Remote browser</b></summary>

Connect to an already-running Chromium instance instead of launching one locally.

```bash
# On the remote machine
chromium --headless --remote-debugging-port=9222 --remote-debugging-address=0.0.0.0 --no-sandbox

# Auto-discover the WebSocket endpoint (recommended)
hxn --remote-host 192.168.1.42:9222 -f urls.txt

# Or pass the WebSocket URL directly
hxn --remote-url ws://192.168.1.42:9222/devtools/browser/<uuid> -u https://example.com
```
</details>

<details> <summary><b>Tab pooling</b></summary>

Tabs are reused by default. Pages are reset between URLs instead of being created and destroyed each time.

```bash
# Custom pool size (default: 8)
hxn -b $(which brave) -f urls.txt --pool-size 16

# Disable tab reuse
hxn -b $(which brave) -f urls.txt --reuse-tabs false
```
</details>

<details> <summary><b>Screenshots</b></summary>

```bash
# Fullpage
hxn -b $(which brave) -u https://example.com --fullpage

# JPEG format
hxn -b $(which brave) -u https://example.com --screenshot-type jpeg

# Custom viewport
hxn -b $(which brave) -u https://example.com -x 1920 -y 1080
```
</details>

<details> <summary><b>Advanced</b></summary>

```bash
# Parallel tabs
hxn -b $(which brave) -f urls.txt --tabs 8

# With ports
hxn -b $(which brave) -f urls.txt --ports 8080,8081

# Proxy
hxn -b $(which brave) -u https://example.com --proxy socks5://127.0.0.1:9050

# Random user agent
hxn -b $(which brave) -f urls.txt --user-agent "random"

# Run JavaScript before screenshot
hxn -b $(which brave) -u https://example.com --javascript "document.querySelector('.banner')?.remove()"

# NDJSON output
hxn -b $(which brave) -f urls.txt --json

# Silent mode
hxn -b $(which brave) -f urls.txt --silent

# Timeout and delay
hxn -b $(which brave) -f urls.txt --timeout 200 --delay 5

# Accept invalid certs
hxn -b $(which brave) -f urls.txt --accept-invalid-certs
```
</details>

---

## Benchmarking

Comparison with [gowitness](https://github.com/sensepost/gowitness) on GitHub Actions (`ubuntu-latest`), measured with [hyperfine](https://github.com/sharkdp/hyperfine). [Full run](https://github.com/pwnwriter/haylxon/actions/runs/27185704378/job/80254042002).

| Benchmark | hxn | gowitness | Result |
|---|---|---|---|
| Single URL | 360ms | 3.99s | **hxn 11x faster** |
| 5 URLs, 4 tabs | 1.81s | 26.29s | **hxn 14x faster** |
| 5 URLs, 8 tabs | 1.70s | 14.67s | **hxn 8x faster** |

---

## Contributing

Contributions welcome via [issues](https://github.com/pwnwriter/haylxon/issues) or [pull requests](https://github.com/pwnwriter/haylxon/pulls).

## Also see

- [gowitness](https://github.com/sensepost/gowitness) — web screenshot utility in Go
- [Kanha](https://github.com/pwnwriter/kanha) — web-app pentesting suite in Rust

## License

[MIT](LICENSE)

<p align="center"><img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/footers/gray0_ctp_on_line.svg?sanitize=true" /></p>
<p align="center">Copyright &copy; 2023 - present <a href="https://pwnwriter.me" target="_blank">pwnwriter</a></p>
