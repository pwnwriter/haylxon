<img src="https://raw.githubusercontent.com/pwnwriter/haylxon/showcase/hxn-transparent.png"  width="240" align="right">

# [`Haylxon`](https://github.com/pwnwriter/haylxon) 🔥🦀
`SHOOT BEFORE THE BLINK` ||  [`Haylxon`](https://github.com/pwnwriter/haylxon/),&nbsp; A tool embodying the [`K1SS`](https://en.wikipedia.org/wiki/KISS_principle) philosophy that allows you to take screenshots of `webpages/URLs` at lightning-fast speeds using `chromes` [`Headless`](https://en.wikipedia.org/wiki/Headless) feature, means, you'd be needing a [***`chromium based browser`***](https://en.wikipedia.org/wiki/Chromium_(web_browser)) for it to work.

Built from the ground up for ease of use, performance, beautiful ui and portability in mind. 💖

<p align="left">

<a href="https://github.com/pwnwriter/haylxon/releases"><img src="https://img.shields.io/github/v/release/pwnwriter/haylxon?style=flat&amp;labelColor=56534b&amp;color=c1c1b6&amp;logo=GitHub&amp;logoColor=white" alt="GitHub Release"></a>
<a href="https://crates.io/crates/hxn/"><img src="https://img.shields.io/crates/v/hxn?style=flat&amp;labelColor=56534b&amp;color=c1c1b6&amp;logo=Rust&amp;logoColor=white" alt="Crate Release"></a>
<a href="https://github.com/pwnwriter/haylxon/blob/main/LICENSE"><img src="https://img.shields.io/badge/License-MIT-white.svg" alt="MIT LICENSE"></a>
<a href="https://twitter.com/intent/tweet?text=Guys,%20Check%20out%20haylxon%20-%20A%20blazingly%20fast%20tool%20to%20grab%20screenshots%20of%20website%2FURL%20from%20terminal%20written%20in%20Rust!%20&url=https%3A%2F%2Fgithub.com%2Fpwnwriter%2Fhaylxon"><img alt="Twitter Share" src="https://img.shields.io/twitter/url/https/github.com/pwnwriter/haylxon.svg?style=social"></a>
[![ko-fi](https://img.shields.io/badge/support-pwnwriter%20-pink?logo=kofi&logoColor=white)](https://ko-fi.com/pwnwriter)

 
![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/aqua.png)


## Table of contents 📔

* [`Features`](#features)
* [`Installation`](#installation)
* [`Hxn in Action`](#action)
* [`Benchmarking`](#benchmarking)
* [`Contribution`](#contribution)
* [`Support`](#support)
* [`Also see`](#see)
* [`License`](#license)
* [`FAQ`](#faq)



![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/aqua.png)

<a name="features"></a>
## Features 🍙
- **Fast**: [`Hxn`](https://github.com/pwnwriter/haylxon) is designed to be fast, all credits goes to the [`rustlang`](https://rust-lang.org). 🦀
- **Portable**: You can use [`haylxon`](https://github.com/pwnwriter/haylxon/) on both [`*nix`](https://en.wikipedia.org/wiki/Linux) & [`windows`](https://en.wikipedia.org/wiki/Microsoft_Windows).
- **Ease**: This tool is designed to be very user friendly as there are very few options but does all the required works that one expects.👨‍🎨
- **Simple**: As always [`Keeping It Simple and Stupid`](https://en.wikipedia.org/wiki/KISS_principle)
 

<a name="installation"></a>
 ## Installation 📩
    
  <details> <summary><code>🪄 Binary </code></summary>
    &nbsp;
   
   -  **Manual**: You can directly download the binary from [**releases**](https://github.com/pwnwriter/haylxon/releases) of your arch and run it.
   - **One liner**: Run this one liner script 

```bash
wget -qO- "$(curl -qfsSL "https://api.github.com/repos/pwnwriter/haylxon/releases/latest" | jq -r '.assets[].browser_download_url' | grep -Ei "$(uname -m).*$(uname -s).*musl" | grep -v "\.sha")" | tar -xzf - --strip-components=1 && ./hxn -h
```  
  </details>
  <details> <summary><code>🌼 Source </code></summary>
  &nbsp;
 
  ```bash
  git clone --depth=1 https://github.com/pwnwriter/haylxon --branch=main
  cd haylxon
  cargo build --release 
  ```
  Then go to `release` dir and `./hxn` or move the `binary` to your any `$PATH` for instant access from anywhere.
</details>

<details> <summary><code>🎠 Cargo </code></summary>

- Using [crates.io](https://crates.io/crates/hxn)
  ```bash
  cargo install hxn
  ```
- Using [binstall](https://github.com/cargo-bins/cargo-binstall)
  ```bash
  cargo binstall hxn
  ```

  > **Note** ⚠️
  > This requires a working setup of rust/cargo & binstall.
</details>

<details> <summary><code>🚩 METIS Linux </code></summary>
&nbsp;
  
  ```bash
  sudo/doas pacman -Syyy hxn
  ```

</details>

<details> <summary><code>💢 Arch user repository </code></summary>
&nbsp;
  
  ```bash
  paru/yay -S haylxon-git
  ```

</details>

<details> <summary><code>❄️ On Nix  </code></summary>
&nbsp;
  
  ```bash
# Build from source and run
  nix run github:pwnwriter/haylxon
# without flakes:
 nix-env -iA nixpkgs.haylxon
# with flakes:
 nix profile install nixpkgs#haylxon
  ```

</details>

  
![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/aqua.png)
  
  
 <a name="action"></a>
## Hxn in action 🚀

  ***I'm using brave browser for all the demonstration. You can use any chromium based browsers.***
  &nbsp;
- <details> <summary><code> Single url </code></summary>
   &nbsp;

  ```bash
  hxn -b $(which brave) -u https://example.com
  ```

</details>

- <details> <summary><code> Multiple urls from file </code></summary>
   &nbsp;

  ```bash
  hxn -b $(which brave) -f urls.txt
  ```

</details>

- <details> <summary><code> Read urls from stdin </code></summary>
  &nbsp;
  
   ```bash
   cat urls.txt | hxn -b $(which brave) --stdin
   ```

</details>

- <details> <summary><code> Custom dimensions </code></summary>
  &nbsp;
 
   ```bash
   cat urls.txt | hxn -b $(which brave) -x 144 -y 400 --stdin
   ```

</details>


- <details> <summary><code> With ports </code></summary>
  &nbsp;
 
   ```bash
   hxn -b $(which brave) -f urls.txt --ports 8080,8081
   ```

</details>

- <details> <summary><code> Arbitrary javascript </code></summary>
    &nbsp;

   ```bash
   hxn -b $(which brave) -u <url> --javascript "javascript code here"
   ```

</details>

- <details> <summary><code> Fullpage screenshots </code></summary>
    &nbsp;

   ```bash
   hxn -b $(which brave) -u https://example.com --fullpage
   ```

</details>

- <details> <summary><code> Screenshot format </code></summary>
    &nbsp;

   ```bash
   hxn -b $(which brave) -u https://example.com --screenshot-type jpeg
   ```

</details>

- <details> <summary><code> Proxy </code></summary>
    &nbsp;

   ```bash
   hxn -b $(which brave) -u https://example.com --proxy socks5://127.0.0.1:9050
   ```

</details>

- <details> <summary><code> Custom user agent </code></summary>
    &nbsp;

   ```bash
   hxn -b $(which brave) -f urls.txt --user-agent "random"
   ```

</details>

- <details> <summary><code> JSON output </code></summary>
    &nbsp;

   ```bash
   hxn -b $(which brave) -f urls.txt --json
   ```

</details>

- <details> <summary><code> Silent mode </code></summary>
    &nbsp;

   ```bash
   hxn -b $(which brave) -f urls.txt --silent
   ```

</details>

- <details> <summary><code> Timeout and delay </code></summary>
    &nbsp;

   ```bash
   hxn -b $(which brave) -f urls.txt --timeout 200 --delay 5
   ```

</details>

- <details> <summary><code> Parallel tabs </code></summary>
    &nbsp;

   ```bash
   hxn -b $(which brave) -f urls.txt --tabs 8
   ```

</details>

- <details> <summary><code> Accept invalid certs </code></summary>
    &nbsp;

   ```bash
   hxn -b $(which brave) -f urls.txt --accept-invalid-certs
   ```

</details>

- <details> <summary><code> Connect to a remote browser </code></summary>
    &nbsp;

   Start a headless Chromium instance on any machine:
   ```bash
   chromium --headless --remote-debugging-port=9222 --remote-debugging-address=0.0.0.0 --no-sandbox
   ```

   Connect via auto-discovery (recommended):
   ```bash
   hxn --remote-host 192.168.1.42:9222 -f urls.txt
   ```

   Or connect via a direct WebSocket URL:
   ```bash
   hxn --remote-url ws://192.168.1.42:9222/devtools/browser/<uuid> -u https://example.com
   ```

</details>

- <details> <summary><code> Tab pooling </code></summary>
    &nbsp;

   Reuse browser tabs instead of creating new ones per URL (enabled by default):
   ```bash
   hxn -b $(which brave) -f urls.txt --pool-size 16
   ```

   Disable tab reuse:
   ```bash
   hxn -b $(which brave) -f urls.txt --reuse-tabs false
   ```

</details>

- <details> <summary><code> hxn in termux </code></summary>
    &nbsp;
   Install dependencies - 
    &nbsp;
   
   ```bash
   pkg install tur-repo -y ; pkg install chromium -y
   ln -s "$PREFIX/bin/chromium-browser" "$PREFIX/bin/chromium"
   ```

</details>


 <a name="benchmarking"></a>
## Benchmarking ⚡
  The below is a comparison between similar project [`gowitness`](https://github.com/sensepost/gowitness), tested on my [**pentium processor**](https://raw.githubusercontent.com/pwnwriter/haylxon/showcase/conf.png).
  
  ![](https://raw.githubusercontent.com/pwnwriter/haylxon/showcase/benchmark.png)
  
  ![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/aqua.png)

  
<a name="contribution"></a> 
## Contribution 🤝

Contributions are welcome! You can suggest features, report bugs, fix issues via [issues](https://github.com/pwnwriter/haylxon/issues) or [pull requests](https://github.com/pwnwriter/haylxon/pulls). Help with code documentation and spreading the word about `HAYLXON(hxn)` is appreciated! 😎

<a name="support"></a>
## Support 💌

 I am a student currently attending university. I like working for *Open Source* in my free time. If you find my tool or work beneficial, please consider supporting me via [*KO-FI*](https://ko-fi.com/pwnwriter) or [*ESEWA*](https://metislinux.org/docs/donate)* (***Nepal only***), Or by leaving a star ⭐ ; I'll appreciate your action :)

<a name="see"></a> 
## Also see 👀
- [`Kanha`](https://github.com/pwnwriter/kanha) :- A web-app pentesting suite written in rust 🦀
- [`gowitness`](https://github.com/sensepost/gowitness) :- A golang, web screenshot utility using Chrome Headless

<a name="faq"></a> 
## FAQ 🥶
 - How do you use `hxn`?
   - I use hxn to automate tasks. For example, I needed to test a website on GitHub Actions before deploying. Using hxn, I took screenshots and uploaded it to    [`0x0.st`](https://0x0.st) to verify it met my requirements.
   - When i have a list of __sub(domains)__ to test. 

<a name="license"></a> 
## License 🔐
 Licensed under the [**`MIT LICENSE`**](/LICENSE) 

 
<p align="center"><img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/footers/gray0_ctp_on_line.svg?sanitize=true" /></p>
<p align="center">Copyright &copy; 2023 - present <a href="https://pwnwriter.me" target="_blank"> pwnwriter me </a> ☘️</p> 
  
