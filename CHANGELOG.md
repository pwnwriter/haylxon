<h3 align="center"><strong> All changes of this release are documented here. </strong> </h3>

![-----------------------------------------------------](https://github.com/pwnwriter/haylxon/blob/readme-assets/colored.png)

> *Note*: ***The 🚨emoji indicates significant changes in the commit.***

## ☃️ Miscellaneous Tasks

- [`2c2ac71`](https://github.com/pwnwriter/hysp/commit/2c2ac71a870b66548171068efdeb0e6727deb7c1): Removed unused devshell pkgs.

- [`1d894e0`](https://github.com/pwnwriter/hysp/commit/1d894e05d1cd583f6c41facece670b46885968bc): Updated website link.

- [`97d7166`](https://github.com/pwnwriter/hysp/commit/97d7166506b1414f46b456f2850ac9f0aac67f09): Ignore direnv dir.

- [`1cce9be`](https://github.com/pwnwriter/hysp/commit/1cce9bebe6b722d7aeca6bffc6c681121dea3601): Show no. of urls in verbose mode.

- [`063a1f6`](https://github.com/pwnwriter/hysp/commit/063a1f6485d672fee208ddbbd670391a865dcc21): Darwin we do not need darwin setup no more.

- [`1174843`](https://github.com/pwnwriter/hysp/commit/11748438ccb43440ca503c55287f5e9b7923d58d): Show clap help menu on no possible arg.

- [`e809170`](https://github.com/pwnwriter/hysp/commit/e809170414c35e1f70118ec63f975f59c9928020): Show full error when obstruct.

- [`9c7ba12`](https://github.com/pwnwriter/hysp/commit/9c7ba124261aefd28804647aac8fbcbd60d416a1): Open blank page properly.

- [`c494356`](https://github.com/pwnwriter/hysp/commit/c494356242c4a4032f8e1dd8fa5fcb8fd6cb674b): Get available options from clap itself don't hardcode.


## ✨Refactor

- [`d5d2dc1`](https://github.com/pwnwriter/hysp/commit/d5d2dc1ebf7f9795efab49681b2d6ebf7be13459): Use nix for devenv, ci workflows.

- [`b242c8e`](https://github.com/pwnwriter/hysp/commit/b242c8e759b197c0d18a2d37048b2d3aa6b9d57c): Fixed syntax and indentation.

- [`4023f34`](https://github.com/pwnwriter/hysp/commit/4023f3495b0db820967a0dc1feb17e0805d231c1): Fixed flake for default devshell ;  ci for tests.

- [`d708fcb`](https://github.com/pwnwriter/hysp/commit/d708fcb0b66ceaa3e84a0c5cd74f1907ce6943ab): Use miette instead of anyhow.

- [`0e6a2a9`](https://github.com/pwnwriter/hysp/commit/0e6a2a9c038e1ca994742294a5614b00c1da3615): Split core logic into different parts, new ascii.

- [`6fb59b7`](https://github.com/pwnwriter/hysp/commit/6fb59b7a82a6716511477a62f0b1287ecaccd206): Cargo-fmt all the way.


## 🐶Performance

- [`b7b1c66`](https://github.com/pwnwriter/hysp/commit/b7b1c66d77de734c5e87876456eb57f25480770e): Use determinate system's nix cache for faster build.


## 👨🏻‍🔧**Bug Fixes**

- [`f0d2a2f`](https://github.com/pwnwriter/hysp/commit/f0d2a2f679ae672bea7087c88248548e07b5270d): Resolve "oneshot canceled" by not breaking handler loop on transient errors.


## 🦁Features

- [`fc62fd0`](https://github.com/pwnwriter/hysp/commit/fc62fd0d4e9a9cc0b938ab6530f5102d67c727f0): Use direnv for faster nav.

- [`06cee2b`](https://github.com/pwnwriter/hysp/commit/06cee2bc66906f3a2f772e2615c034af68588fc1): Nix default package/shell for the app.

- [`f5903dc`](https://github.com/pwnwriter/hysp/commit/f5903dc1401647389ac4555bbc07f9d97c5f21e1): Docs for running on nix.

- [`1a6996f`](https://github.com/pwnwriter/hysp/commit/1a6996f52bbd765f831899c3567f83f7feb79e88): Docs for flakes, without flakes usages for nix.

- [`4f7f597`](https://github.com/pwnwriter/hysp/commit/4f7f597513e869ec206b28267ad3c1147c3e6b4d): Show table like ui for urls ss output.

- [`8aa4255`](https://github.com/pwnwriter/hysp/commit/8aa4255e63209ca75b70731ac27321ea94e5adf9): Add support for proxy.

- [`02eedf1`](https://github.com/pwnwriter/hysp/commit/02eedf1aaab848c4e8de35c2337f96ca1fdf7531): Add support for output in json format.

- [`bd943dc`](https://github.com/pwnwriter/hysp/commit/bd943dcdd48199bf3a7fcf70703edaa8d3f145c0): Handle urls protocol before taking ss, normalize ss taking.

- [`897ef4f`](https://github.com/pwnwriter/hysp/commit/897ef4f84a409db1ffa16b541fc8925d3f050f67): Lsp config using lazy nvim.

- [`a38a28c`](https://github.com/pwnwriter/hysp/commit/a38a28c1f45498ef277db54ef4cae467a2c6aa7b): New features usages and follow up commands.


## ☃️ Miscellaneous Tasks

- [`4ee2bb4`](https://github.com/pwnwriter/hysp/commit/4ee2bb494a7cf2379ddf5d68a283b6b717004b4c): Fixed typo for v. release ; use 5 max prs for deps.

- [`462669c`](https://github.com/pwnwriter/hysp/commit/462669c1a4c99c98026c41a2cadbfc47df01c46f): Help for possible port values type.

- [`f44a7a9`](https://github.com/pwnwriter/hysp/commit/f44a7a9e231d64497075e4c0acae0aa97dc1f5d8): Instantiate libenv inside flake.

- [`b156a9f`](https://github.com/pwnwriter/hysp/commit/b156a9f428f7da09ee516ec3a0bf9c5c4af8e11c): Removed no trust dns to accept invalid certs.

- [`4f67059`](https://github.com/pwnwriter/hysp/commit/4f670597bdc95538c6a4ef8cf6ce0e87e8b4cfee): Check if the ss was successful or not and then only log.


## 🦁Features

- [`bfe92f0`](https://github.com/pwnwriter/hysp/commit/bfe92f0d07ffe70610863521ce18a450f43ac80a): Allow parsing ports in a sequence [closes #62].

- [`2ef2516`](https://github.com/pwnwriter/hysp/commit/2ef2516c6dc9e36a82510a54071e975ec96ec37b): Use info logger to show screenshot msgs; handle error cases.

- [`be484c7`](https://github.com/pwnwriter/hysp/commit/be484c78aeffa9a85954023b05fcaaf0d122185c): Tests functions for ports; fixed typos.

- [`4d5d52f`](https://github.com/pwnwriter/hysp/commit/4d5d52fd769eeedb76325804d02b37d77edb6351): Allow defining invalid in 'args'.

- [`1bc7a7d`](https://github.com/pwnwriter/hysp/commit/1bc7a7db20ca4db06da40afc3661139c944d09af): Dev flake for cc build.

- [`53987da`](https://github.com/pwnwriter/hysp/commit/53987da09f99868f4f8b69a2057ee934a4482010): Run arbitary javascript in websites.

- [`cee4172`](https://github.com/pwnwriter/hysp/commit/cee4172cadfceb0e16ec4898ae5e53b7774c0fbe): Guide on arbitary js option.

- [`d0b5f1b`](https://github.com/pwnwriter/hysp/commit/d0b5f1be98f30b535cf6e978d650e22ea47eab29): Faq section and answers.

- [`d78c741`](https://github.com/pwnwriter/hysp/commit/d78c741998f6c4c39197f5cb533d992f247d3cf5): Release notes for new release.

- [`0381a15`](https://github.com/pwnwriter/hysp/commit/0381a15739ac3c07523a315878cd8fe505bf956f): Release new version.


## ☃️ Miscellaneous Tasks

- [`7af9026`](https://github.com/pwnwriter/hysp/commit/7af9026749b780638a8b21474093850b4490bd7f): Removed bench with other apps.

- [`09c7451`](https://github.com/pwnwriter/hysp/commit/09c74517fee095447e546e2c77793b1a7ff26f1f): Formated with cargo-fmt.

- [`aa452bf`](https://github.com/pwnwriter/hysp/commit/aa452bfda4a1d86db6da43f84b61498699e19db7): Show demo ; deps for termux (android).

- [`73b4ed2`](https://github.com/pwnwriter/hysp/commit/73b4ed215eaaa8b71c925db1f93174be4dfeeda3): New ascii splash for ui.

- [`87ff21e`](https://github.com/pwnwriter/hysp/commit/87ff21efa3bdb73f8dce361b640a1461cc2ef12c): Temp pausing domain testing workflow.

- [`3a22c24`](https://github.com/pwnwriter/hysp/commit/3a22c240c6498b036fe6ff3e5360e6be2953904a): Added one liner script for bin installation.

- [`fd7b118`](https://github.com/pwnwriter/hysp/commit/fd7b118bfab8ec4f6ea972d482708d9a07ef72f8): Show manual // one liner bin installation seprately.

- [`89db2c2`](https://github.com/pwnwriter/hysp/commit/89db2c26ba398ecd4b543a55c1ae3aabb54849cc): Added support links.


## 🦁Features

- [`3967d8a`](https://github.com/pwnwriter/hysp/commit/3967d8a1e50f1dfc89af1112514ea8843d731df9): Options to change image type,size [closes #54].

- [`4788272`](https://github.com/pwnwriter/hysp/commit/47882720cd6330ba5e465aac7f2a4d5ccb0caedd): Use seprate config, updated changelogs notes.

- [`89707cf`](https://github.com/pwnwriter/hysp/commit/89707cf5a270df50d885816e5bf29ea16c40ccbc): Option to provide `port` values [closes #60].

- [`ae5b8ab`](https://github.com/pwnwriter/hysp/commit/ae5b8ab831e3052b5e7e1a0079e910e936d240e1): Notes for new version(0.1.10) release.

- [`aa87582`](https://github.com/pwnwriter/hysp/commit/aa87582b04cd670c01c514d156e964dd481a3b82): Manual for defining port values.


## ☃️ Miscellaneous Tasks

- [`910be93`](https://github.com/pwnwriter/hysp/commit/910be93ba4ae4b089432f410445e06a24786318e): Add AUR installation guide.

- [`4e7dffa`](https://github.com/pwnwriter/hysp/commit/4e7dffae7faed2ac02a9c5318044c142d630c37d): Show quote inside ascii block.

- [`fc52b80`](https://github.com/pwnwriter/hysp/commit/fc52b805ad75462ed146f1cbd085ccaa03706be4): [closes #22] ignore danger certs // dns.

- [`116c57b`](https://github.com/pwnwriter/hysp/commit/116c57b6242f4ae14bdc07e9ded1aa908401fd7f): Use gh actions to test domains.

- [`0e35c09`](https://github.com/pwnwriter/hysp/commit/0e35c0957f93cc6ddaf5fc465b304db6a5530efe): Use a long timeout to preserve domain ss.

- [`09b50f7`](https://github.com/pwnwriter/hysp/commit/09b50f7afce0021be65cd3675f2651799579a090): Use long timeout // redirect msgs while installing deps.

- [`abfaca6`](https://github.com/pwnwriter/hysp/commit/abfaca60c7c5e047798bfc56586f91230d4c49e0): Use multiple tabs to increase speed.

- [`9fa08df`](https://github.com/pwnwriter/hysp/commit/9fa08dfec7ca1072e163a4a9402bf119c3617cd1): Test domains screenshots on the test commit.

- [`607c007`](https://github.com/pwnwriter/hysp/commit/607c007ca998ffe399f41783a62f053aed9019d5): Fixed typos to trigger action.

- [`217dccb`](https://github.com/pwnwriter/hysp/commit/217dccba6cceb59c42bed93cdaafefcbc339386a): Trigger action on push.

- [`4efd745`](https://github.com/pwnwriter/hysp/commit/4efd745963da6ee6740b6a4554bd25abb88b7e44): Use multple tabs // timeout.

- [`16a5e51`](https://github.com/pwnwriter/hysp/commit/16a5e51503dc03e568b78b196a6779462ef42a4c): Manually install hyprfine than using binstall.

- [`d89cf20`](https://github.com/pwnwriter/hysp/commit/d89cf20803b73f4bd1f40d49712eb2687ee536ef): Fixed typos on path.

- [`f9a25b8`](https://github.com/pwnwriter/hysp/commit/f9a25b8ed90073c436b6bcfe1dcab09cac935d7a): Fixed typosss.

- [`bf43844`](https://github.com/pwnwriter/hysp/commit/bf438447ba98f7f5dc1edf817b8053d005767074): Fixed typo with hyperfine.

- [`50178f4`](https://github.com/pwnwriter/hysp/commit/50178f459f4609cf853730d589ede15913f79ff6): Removed out dir for gowitness.


## ✨Refactor

- [`d15d64c`](https://github.com/pwnwriter/hysp/commit/d15d64c07b3a0b5d8961671dc086a0a80d52bdec): Use verbose mode to grab info else don't even make a request.

- [`4717886`](https://github.com/pwnwriter/hysp/commit/4717886d5f17bd13f3b94bbe5a96b193298a9d5c): Run domain tests on seprate job.

- [`6e5ceb1`](https://github.com/pwnwriter/hysp/commit/6e5ceb1813cd6cef8c2a532e3314120b82d8c51f): Soo, it's bash not zsh.

- [`67e7ef4`](https://github.com/pwnwriter/hysp/commit/67e7ef419663122ce6ae3c5a9b268d314108b732): Formated with carg-fmt // indented.


## 🦁Features

- [`b43753f`](https://github.com/pwnwriter/hysp/commit/b43753fc273fe0c8e7e3935c74e3128bfee5692c): [closes #34], Take full page screenshots of urls.

- [`e111ed6`](https://github.com/pwnwriter/hysp/commit/e111ed68e9e083aae147ed05c73ab89a5ea65e4f): Bench gowitness with hxn via ci/c.

- [`60d71ef`](https://github.com/pwnwriter/hysp/commit/60d71ef3b7b3b8e368b6709a09e60a850c7745f3): Demos for new commands.

- [`f1b13b9`](https://github.com/pwnwriter/hysp/commit/f1b13b91781c8effda69bd17679e6f670e6a2b24): Preparing for new [v0.1.9] release.


## ☃️ Miscellaneous Tasks

- [`b062793`](https://github.com/pwnwriter/hysp/commit/b062793a20d85382b34180c7c0f904ef6e0bb437): Print no title if url doesn't have one // closes #30.


## 🦁Features

- [`21b9755`](https://github.com/pwnwriter/hysp/commit/21b9755d8bc048c32def68fbae28e207609c11f1): Bug-report,feature req templates.

- [`f1dda5f`](https://github.com/pwnwriter/hysp/commit/f1dda5f2841aab34ffd4a00b314d82c88c56fd98): Notes // docx for new release.


## ☃️ Miscellaneous Tasks

- [`dbf3106`](https://github.com/pwnwriter/hysp/commit/dbf3106ff8a6c31df23f2c4e1877fadd967adbb0): Fixed cargo installation method, closes #18.

- [`5dea196`](https://github.com/pwnwriter/hysp/commit/5dea19615bc53338bb933c40f9cab406177dcbf0): Fixed source installation.

- [`dd4d8cd`](https://github.com/pwnwriter/hysp/commit/dd4d8cdd9fb762f4ce15a8d8be1e897173894b21): Dynamically retrieve version from Cargo.toml.

- [`f28577d`](https://github.com/pwnwriter/hysp/commit/f28577d1a9ded242cbb2276c88ebad608bbbd038): Specify new --file-path | -f option.

- [`8c3c31f`](https://github.com/pwnwriter/hysp/commit/8c3c31f2c554d649aca0957bec719ed8c1047c22): Demo for multiple urls // stdin.


## ✨Refactor

- [`78d5a80`](https://github.com/pwnwriter/hysp/commit/78d5a801802774d78f1f4c4c430baa4e45eaa8ac): Enhance code quality and cli module.


## 🦁Features

- [`9c26f9e`](https://github.com/pwnwriter/hysp/commit/9c26f9edfecbfbb1014fee99665a32855c609141): Updated docx for new (v0.1.7) release.


## Documentation

- [`2b2ae95`](https://github.com/pwnwriter/hysp/commit/2b2ae95d873bbaa3362792aac8c6ac46e4aa7ec7): Update.


## ☃️ Miscellaneous Tasks

- [`76ef179`](https://github.com/pwnwriter/hysp/commit/76ef179164dc10ea537d337e484df4c3f67ca39b): Providing info with clap.

- [`c5ca551`](https://github.com/pwnwriter/hysp/commit/c5ca551c0b735930d0469cad877b541c37a3defb): Removed color printings.

- [`cdc3cd3`](https://github.com/pwnwriter/hysp/commit/cdc3cd36aa54b0599df3386347086711aed54310): References to similar projects.

- [`7242d51`](https://github.com/pwnwriter/hysp/commit/7242d5123dec09ffdb53072cae2e9c8e23f6560f): Provide lockfile for locked installation.

- [`8249a4e`](https://github.com/pwnwriter/hysp/commit/8249a4ee18447e2daabccac684a1e94b9e7b1fe7): Disable default features for matrix compilation.

- [`c88d8b0`](https://github.com/pwnwriter/hysp/commit/c88d8b00e7e8c453e49fec66f3e802a0ee3fee82): Use shortened binary name for release artifacts.


## ✨Refactor

- [`0ce2623`](https://github.com/pwnwriter/hysp/commit/0ce2623b12f0bd4c7f1990f50365b81bdd97718f): (args), devided code into multiple files.

- [`2924ce4`](https://github.com/pwnwriter/hysp/commit/2924ce4551e5d76fee206e9e3aff9d6737569a03): Use seprate dirs // feat(args): stdin support.

- [`83cb275`](https://github.com/pwnwriter/hysp/commit/83cb275a53dcdf94048b5df007fe784df3d8d637): Cargo-fmt all the way.


## 🌈 Beautify

- [`2ccdd79`](https://github.com/pwnwriter/hysp/commit/2ccdd79988ad72aa2f163392c7b0e176441840c3): Use colored for colorful output // contents.


## 🌝Testing

- [`c5d1611`](https://github.com/pwnwriter/hysp/commit/c5d1611f2a5031f082701b5edfbb5b1ab680eaac): Testing ci/cd.


## 👨🏻‍🔧**Bug Fixes**

- [`1982893`](https://github.com/pwnwriter/hysp/commit/1982893460df2f9152b2f19994fa79d3e5421647): Aur pkg name.


## 🦁Features

- [`da55ddb`](https://github.com/pwnwriter/hysp/commit/da55ddb07cf48952ccdc3e6cbbdc73415bde1be5): Option for defining timeout.

- [`0bfa8c3`](https://github.com/pwnwriter/hysp/commit/0bfa8c334d221cda3d737a71b4f56dca5e0c0a76): Use ci/cd for binary releases // dependency management.

- [`0125b8a`](https://github.com/pwnwriter/hysp/commit/0125b8a28264994b2dd7b2f66b303fb9fc9190aa): Demos for the latest releases.

- [`64d0c0c`](https://github.com/pwnwriter/hysp/commit/64d0c0c35b026911b3518e9b1300d4b5b12fad69): Release notes // version updates for releases.


## ✨Refactor

- [`9fa22cd`](https://github.com/pwnwriter/hysp/commit/9fa22cdb35e1325a22c0fb4f69d73e0862e41fbe): 🪛 use function to exit.


## 👨🏻‍🔧**Bug Fixes**

- [`34ffe14`](https://github.com/pwnwriter/hysp/commit/34ffe148186ec1e9be09a4af25c519b904a6fa3a): Panicking while no browser binary present.


## ☃️ Miscellaneous Tasks

- [`d7b257e`](https://github.com/pwnwriter/hysp/commit/d7b257efc7aac8a05be5d0312bed324bfd28e104): Optimized 📦 build/released/dev + provide 🗝  lock file anyway.


## ✨Refactor

- [`899acc9`](https://github.com/pwnwriter/hysp/commit/899acc9c05f8465e8c674d0c7649999eaa5675a6): Make it work with Clap v4.


## 👨🏻‍🔧**Bug Fixes**

- [`f873c4f`](https://github.com/pwnwriter/hysp/commit/f873c4f6cd69e906fc200f7198046d2021c40aed): Binary name + git links.

<!-- generated by [`git-cliff`](https://github.com/orhun/git-cliff) -->
<p align="center"><img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/footers/gray0_ctp_on_line.svg?sanitize=true" /></p>
