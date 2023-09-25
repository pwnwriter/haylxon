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
* [`License`](#license)
* [`Also see`](#see)

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/aqua.png)

<a name="features"></a>
## Features 🍙
- **Fast**: [`Hxn`](https://github.com/pwnwriter/haylxon) is designed to be fast, all credits goes to the [`rustlang`](https://rust-lang.org). 🦀
- **Portable**: You can use [`haylxon`](https://github.com/pwnwriter/haylxon/) on both [`*nix`](https://en.wikipedia.org/wiki/Linux) & [`windows`](https://en.wikipedia.org/wiki/Microsoft_Windows).
- **Ease**: This tool is designed to be very user friendly as there are very few options but does all the required works that one expects.👨‍🎨
- **Simple**: As always [`Keeping It Simple and Stupid`](https://en.wikipedia.org/wiki/KISS_principle) 💋
 

<a name="installation"></a>
 ## Installation 📩
    
  <details> <summary><code>🪄 Binary </code></summary>
    &nbsp;

  - You can directly download the [**binary**](https://github.com/pwnwriter/haylxon/releases) of your arch and run it.
  
  </details>
  <details> <summary><code>🌼 Source </code></summary>
  &nbsp;
 
  ```bash
  git clone --depth=1 https://github.com/pwnwriter/haylxon --branch=main
  cd haylxon
  cargo build --release 
  ```
  Then go to `release` dir and `./haylxon` or move the `binary` to your any `$PATH` for instant access from anywhere.
</details>

<details> <summary><code>🎠 Cargo </code></summary>

- Using [crates.io](https://crates.io/crates/haylxon)
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
  sudo/doas pacman -Syyy haylxon
  ```

</details>
  
![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/aqua.png)
  
  
 <a name="action"></a>
## Hxn in action 🚀

- <details> <summary><code> Taking screenshots of one/many urls.🖇️ </code></summary>

 
  - single url
  
   &nbsp;
  ***I'm using brave browser for all the demonstration. You can use any chromium based browsers.***
 
  ```bash
  hxn -b $(which brave) -u https://example.com
  ```
  ![single](https://github.com/pwnwriter/haylxon/assets/90331517/6b6460f8-72dd-4197-b43f-f283a2c49727)

  - a file containing multiple urls

    &nbsp;

  ```bash
  hxn -b $(which brave) -u urls.txt
  ```

  ![many](https://github.com/pwnwriter/haylxon/assets/90331517/86f987d9-0961-4247-841e-18aee6aaf53f)

</details>


- <details> <summary><code> Silent option for automation ⚙️ </code></summary>
   &nbsp;
  
   ```bash
   hxn -b $(which brave) -u https://example.com --silent
   ```
 
  ![silent](https://github.com/pwnwriter/haylxon/assets/90331517/80e9f15f-9087-4b9f-a1d6-3e631e795b9d)

</details>

- <details> <summary><code> Read urls from stdin ⚓ </code></summary>
  &nbsp;
  
   ```bash
   cat urls.txt | hxn -b $(which brave) --stdin
   ```
  
  ![stdin](https://github.com/pwnwriter/haylxon/assets/90331517/db5b8542-af54-420a-8478-7bef4ef6fe0c)

  
</details>

- <details> <summary><code> Define x/y dimentions 🐀 </code></summary>
  &nbsp;
 
   ```bash
   cat urls.txt | hxn -b $(which brave) -x 144 -y 400 --stdin
   ```
  
  ![dimention](https://github.com/pwnwriter/haylxon/assets/90331517/c436100e-d647-40b2-9987-f52f81e09490)

  
</details>

- <details> <summary><code> Help pls 🐤 </code></summary>
    &nbsp;
   
     If your internet is slow, you may need to increase the timeout value for 'hxn' to work properly

  ```bash
   cat urls.txt | hxn -b $(which brave) --timeout 60
   ```
  
  ![timeout_error](https://github.com/pwnwriter/haylxon/assets/90331517/96f69d9a-cfca-4af9-9987-4f336099f3f8)


  
</details>


 <a name="benchmarking"></a>
## Benchmarking ⚡
  The below is a comparison between similar project [`gowitness`](https://github.com/sensepost/gowitness), tested on my [**pentium processor**](https://raw.githubusercontent.com/pwnwriter/haylxon/showcase/conf.png).
  
  ![](https://raw.githubusercontent.com/pwnwriter/haylxon/showcase/benchmark.png)
  
  ![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/aqua.png)

  
<a name="contribution"></a> 
## Contribution 🤝
  There is always scope for improvements and bugs to be fixed as contributors can make a valuable impact by addressing improvements and bugs through [**issue**](https://github.com/pwnwriter/haylxon/issues) submissions or [**pull requests**](https://github.com/pwnwriter/haylxon/pulls).
  
  - Still, here are the ways to contribute,
    - Suggest a feature
    - Report a bug
    - Fix something and open a pull request
    - Help me document the code and more speed pleaseeeeeee.
    - Spread the word [**`HAYLXON(hxn)`**](https://github.com/pwnwriter/haylxon) 😎
   
<a name="license"></a> 
## License 🔐
 As always, this project is also Licensed under the [**`MIT LICENSE`**](/LICENSE) 

<a name="see"></a> 
## Also see 👀
- [`Kanha`](https://github.com/pwnwriter/kanha) :- A web-app pentesting suite written in rust 🦀
- [`gowitness`](https://github.com/sensepost/gowitness) :- A golang, web screenshot utility using Chrome Headless
 
<p align="center"><img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/footers/gray0_ctp_on_line.svg?sanitize=true" /></p>
<p align="center">Copyright &copy; 2023<a href="https://pwnwriter.xyz" target="_blank"> pwnwriter xyz </a> ☘️</p> 
  
