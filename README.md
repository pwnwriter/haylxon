<img src="https://raw.githubusercontent.com/pwnwriter/haylxon/showcase/hxn-transparent.png" alt="binserve logo" width="240" align="right">

# [`Haylxon`](https://github.com/pwnwriter/haylxon) 🔥🦀
`SHOOT BEFORE THE BLINK` ||  [`Haylxon`](https://github.com/pwnwriter/haylxon/),&nbsp; A tool embodying the [`K1SS`](https://en.wikipedia.org/wiki/KISS_principle) philosophy that allows you to take screenshots of `webpages/URLs` at lightning-fast speeds.

Built from the ground up for ease of use, performance, beautiful ui and portability in mind. 💖

<p align="left">

<a href="https://github.com/pwnwriter/haylxon/releases"><img src="https://img.shields.io/github/v/release/pwnwriter/haylxon?style=flat&amp;labelColor=56534b&amp;color=c1c1b6&amp;logo=GitHub&amp;logoColor=white" alt="GitHub Release"></a>
<a href="https://crates.io/crates/hxn/"><img src="https://img.shields.io/crates/v/hxn?style=flat&amp;labelColor=56534b&amp;color=c1c1b6&amp;logo=Rust&amp;logoColor=white" alt="Crate Release"></a>
<a href="https://github.com/pwnwriter/haylxon/blob/main/LICENSE"><img src="https://img.shields.io/badge/License-MIT-white.svg" alt="MIT LICENSE"></a>
<a href="https://twitter.com/intent/tweet?text=Guys,%20Check%20out%20haylxon%20-%20A%20blazingly%20fast%20tool%20to%20grab%20screenshots%20of%20website%2FURL%20from%20terminal%20written%20in%20Rust!%20&url=https%3A%2F%2Fgithub.com%2Fpwnwriter%2Fhaylxon"><img alt="Twitter Share" src="https://img.shields.io/twitter/url/https/github.com/pwnwriter/haylxon.svg?style=social"></a>
![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/aqua.png)


## Table of contents 📔

* [`Features`](#features)
* [`Installation`](#installation)
* [`Hxn in Action`](#action)
* [`Benchmarking`](#benchmarking)
* [`Contribution`](#contribution)
* [`License`](#license)
* [`Thanks`](#thanks)

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/aqua.png)

<a name="features"></a>
## Features 🍙
- **Fast**: [`Hxn`](https://github.com/pwnwriter/haylxon) is designed to be fast, all credits goes to the [`rustlang`](https://rust-lang.org). 🦀
- **Portable**: You can use [`haylxon`](https://github.com/pwnwriter/haylxon/) on both [`*nix`](https://en.wikipedia.org/wiki/Linux) & [`windows`](https://en.wikipedia.org/wiki/Microsoft_Windows).
- **Ease**: This tool is designed to be very user friendly as there are very few options but does all the required works that one expects.👨‍🎨
- **Simple**: As always [`Keeping It Simple and Stupid`](https://en.wikipedia.org/wiki/KISS_principle) 💋
 

<a name="installation"></a>
 ## Installation 📩
- **Binary**:
  You can directly download [`binary`](https://github.com/pwnwriter/haylxon/releases/) and run it.
- **Source**:
  ```bash
  $ git clone --depth=1 https://github.com/pwnwriter/haylxon --branch=main
  $ cd haylxon
  $ cargo build --release 
  ```
  Then go to `release` dir and `./hxn` or move the `binary` to your any `$PATH` for instant access from anywhere.
- **Cargo**:
  ```bash
  $ cargo install hxn
  ```
  > Note: This requires a working setup of rust/cargo.
 
- **Aur**:
  ```bash
  $ paru/yay -Syyy hxn
  ```
  
- **[Metis Linux](https://metislinux.org)**:
  ```
  $ sudo/doas pacman -Syyy hxn
  ```
![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/aqua.png)
  
  
 <a name="action"></a>
## Hxn in action 🚀
- Help menu :
  [`Hxn`](https://github.com/pwnwriter/haylxon) utilizes [`clap`](https://docs.rs/clap/latest/clap/) for argument parsing. As expected, `clap` provides a much-improved help menu. The screenshot     below shows Hxn's help menu when the  `-h` flag is used. 
  
  > use `--help` for full menu.
  
 ![](https://raw.githubusercontent.com/pwnwriter/haylxon/showcase/v0.1.2/help.png)
 
 - Taking screenshots of one/many urls.🖇️
 
 **It doesn't matter**, if you are parsing a `single url` or a `file` containing a list of `urls`, [`Hxn`](https://github.com/pwnwriter/haylxon) will handle itself. You don't need to worry about other flags.
  
  `v0.1.2` has an option to define `browser binary` [`#pr`](https://github.com/pwnwriter/haylxon/pull/8), by default it uses `/usr/bin/chrome`.
  
  - **Single URL**
   ```bash
   $ hxn -b $(which <browserbin>) -u <url> 
   ```
    
   ![](https://raw.githubusercontent.com/pwnwriter/haylxon/showcase/v0.1.2/single.png)
    
   - **File containing more than one url**. 🖇️
   ```bash
   $ hxn -b <browserbin> -t <no. of tabs to open //ly> -u <url/filename> -o <outdir(default hxnshots)>
   ```
    
   ![](https://raw.githubusercontent.com/pwnwriter/haylxon/showcase/v0.1.2/many.png)
   
   ![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/aqua.png)

  - **Define x/y** 🐣
   The latest release has an option to set `height`, `width` and `--silent` option.
   
   ```bash
   $ hxn --height <height> --width <widht> --silent -url <url>
   ```
    
   ![](https://raw.githubusercontent.com/pwnwriter/haylxon/showcase/v0.1.2/feature.png)

    

 <a name="benchmarking"></a>
## Benchmarking ⚡
  The below is a comparison between similar project [`gowitness`](https://github.com/sensepost/gowitness), tested on my [**pentium processor**](https://raw.githubusercontent.com/pwnwriter/haylxon/showcase/conf.png).
  
  ![](https://raw.githubusercontent.com/pwnwriter/haylxon/showcase/benchmark.png)
  
  ![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/aqua.png)

  
<a name="contribution"></a> 
## Contribution 🥰
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
  
<a name="thanks"></a>
## Thanks 🌹
  A big thanks to the followings.
 - [`@rustlang`](https://rustlang.org)
 - [`@prabuddha`](https://www.facebook.com/PrabuddhaMP4)
 - [`@Dylan Arps`](https://github.com/dylanaraps)
 - [`@catppuccin`](https://github.com/catppuccin) 
 - [`@cute-ducky`](https://github.com/Cute-Ducky) // plan9boys.
 - [`@blast`](https://github.com/joshua-mo-143)
 - and [**you**](https://github.com/pwnwriter/haylxon/graphs/contributors) ❤️‍🩹
 
 Similar projects : [`gowitness`](https://github.com/sensepost/gowitness), [`eyewitness`](https://github.com/FortyNorthSecurity/EyeWitness)
  
<p align="center"><img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/footers/gray0_ctp_on_line.svg?sanitize=true" /></p>
<p align="center">Copyright &copy; 2023<a href="https://pwnwriter.xyz" target="_blank"> pwnwriter xyz ☘️ </a> 
  
