# Changelog

All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

---
## [0.3.4](https://github.com/lukehsiao/netlify-ddns-rs/compare/v0.3.3..v0.3.4) - 2024-01-19

### Documentation

- **(CHANGELOG)** add entry for v0.3.4 - ([e58caad](https://github.com/lukehsiao/netlify-ddns-rs/commit/e58caad1dd930acd154c3b2c119eedff3f61c966)) - Luke Hsiao
- **(README)** fix badge for github actions - ([ee91cba](https://github.com/lukehsiao/netlify-ddns-rs/commit/ee91cba21fa63680ff274d8a03497500189b2bee)) - Luke Hsiao
- **(README)** update readme style - ([f993240](https://github.com/lukehsiao/netlify-ddns-rs/commit/f993240f5a5c7e84247b242752adc2d752cc504f)) - Luke Hsiao

### Bulid

- **(deps)** upgrade ureq to v2.8.0 - ([c8c25e5](https://github.com/lukehsiao/netlify-ddns-rs/commit/c8c25e5835a25432932ad5aa501aae21832af851)) - Luke Hsiao

---
## [0.3.3](https://github.com/lukehsiao/netlify-ddns-rs/compare/v0.3.2..v0.3.3) - 2023-06-20

### Documentation

- **(CHANGELOG)** add entry for v0.3.3 - ([e8122e9](https://github.com/lukehsiao/netlify-ddns-rs/commit/e8122e9a92c97d7452873df3e8388cb971f1449d)) - Luke Hsiao
- **(README)** update badges - ([062ba30](https://github.com/lukehsiao/netlify-ddns-rs/commit/062ba302ec6ac8d4ad0edfdc66933e758ecc1b12)) - Luke Hsiao

### Refactor

- switch from log to tracing - ([21512b7](https://github.com/lukehsiao/netlify-ddns-rs/commit/21512b7c9598e50511ce90b46e4dba96c508ab90)) - Luke Hsiao

---
## [0.3.2](https://github.com/lukehsiao/netlify-ddns-rs/compare/v0.3.1..v0.3.2) - 2022-11-02

### Bug Fixes

- include TTL in computing conflicts - ([02042b1](https://github.com/lukehsiao/netlify-ddns-rs/commit/02042b1038d567e60e4a2bbb234b154059664a49)) - Luke Hsiao

### Documentation

- **(CHANGELOG)** fix all commits urls - ([a5d47be](https://github.com/lukehsiao/netlify-ddns-rs/commit/a5d47be4ff7f9fa4fd4313f64a2bdf15c1cf71ee)) - Luke Hsiao
- **(CHANGELOG)** add entry for v0.3.2 - ([5e51646](https://github.com/lukehsiao/netlify-ddns-rs/commit/5e51646fcd1a40d8cc1e2fb26607fe8f05b08c27)) - Luke Hsiao
- **(README)** add passively-maintained badge - ([b5d41ed](https://github.com/lukehsiao/netlify-ddns-rs/commit/b5d41ede640cabfd2b19a686af8a434083c19f45)) - Luke Hsiao
- **(README)** link to maintenance status definitions - ([bf864b4](https://github.com/lukehsiao/netlify-ddns-rs/commit/bf864b44e976bd8ef5178bd853e0cc04f0c4c8f3)) - Luke Hsiao

---
## [0.3.1](https://github.com/lukehsiao/netlify-ddns-rs/compare/v0.3.0..v0.3.1) - 2022-09-23

### Documentation

- **(CHANGELOG)** add entry for v0.3.1 - ([3b1c283](https://github.com/lukehsiao/netlify-ddns-rs/commit/3b1c2833d906aec523d0ba58276fdc2080501f21)) - Luke Hsiao
- **(README)** update usage for latest clap output - ([bddc651](https://github.com/lukehsiao/netlify-ddns-rs/commit/bddc651cb2c15d34d238c114ac2e24a99f93df21)) - Luke Hsiao

### Refactor

- fix clippy lint - ([238769a](https://github.com/lukehsiao/netlify-ddns-rs/commit/238769a09f27efe8f8d4a13f518e38909ff24eda)) - Luke Hsiao

### Styling

- fix clippy lints - ([51e673e](https://github.com/lukehsiao/netlify-ddns-rs/commit/51e673e3d301bf6ebf731d5ca36e67a783e123cc)) - Luke Hsiao

---
## [0.3.0](https://github.com/lukehsiao/netlify-ddns-rs/compare/v0.2.4..v0.3.0) - 2021-01-04

### Documentation

- **(CHANGELOG)** update for v0.3.0 - ([d4f0ca2](https://github.com/lukehsiao/netlify-ddns-rs/commit/d4f0ca221c79afa202cd28deda30fca458f5846b)) - Luke Hsiao
- **(README)** add example cronjob and links to related projects - ([a87c09d](https://github.com/lukehsiao/netlify-ddns-rs/commit/a87c09daa352509862d6dd8f3ac19cc643601370)) - Luke Hsiao

### Refactor

- switch from failure to thiserror and anyhow - ([4cf23a2](https://github.com/lukehsiao/netlify-ddns-rs/commit/4cf23a2eef953e91e2fc314c03013b70142ec038)) - Luke Hsiao
-  [**breaking**] update to ureq v2.0.0 - ([93d45ac](https://github.com/lukehsiao/netlify-ddns-rs/commit/93d45acf524ca4233e2a8ff751ce7d038a755ec9)) - Luke Hsiao

### Styling

- fix cargo clippy warnings - ([2724ec1](https://github.com/lukehsiao/netlify-ddns-rs/commit/2724ec15687f1a44c6f5ac9fc4543d61e91e4c46)) - Luke Hsiao

### Testing

- switch from Travis to GitHub Actions - ([400f252](https://github.com/lukehsiao/netlify-ddns-rs/commit/400f25240829b1347f5fbe7ed037bacfa4e17e6f)) - Luke Hsiao

---
## [0.2.4](https://github.com/lukehsiao/netlify-ddns-rs/compare/v0.2.3..v0.2.4) - 2020-05-02

### Documentation

- **(CHANGELOG)** update list for v0.2.4 release - ([2653111](https://github.com/lukehsiao/netlify-ddns-rs/commit/2653111ec36df5a04800cdfe8a793016fba52e46)) - Luke Hsiao

### Features

- extend conflict detection to handle subdomain globs (#14) - ([8d3f508](https://github.com/lukehsiao/netlify-ddns-rs/commit/8d3f50802bc6cf626e49beb3c4e80acba611f850)) - Kevin Smith

### Testing

- allow beta to fail, and use bionic - ([68c884c](https://github.com/lukehsiao/netlify-ddns-rs/commit/68c884c6f7174accdffca4385dbc09400f9137dc)) - Luke Hsiao

---
## [0.2.3](https://github.com/lukehsiao/netlify-ddns-rs/compare/v0.2.2..v0.2.3) - 2020-02-05

### Documentation

- **(README)** drop version from usage example - ([23e405f](https://github.com/lukehsiao/netlify-ddns-rs/commit/23e405f70934c0523a4f62f8dea3db34352dbe77)) - Luke Hsiao
- update changelog and readme for v0.2.3 - ([d1fdd94](https://github.com/lukehsiao/netlify-ddns-rs/commit/d1fdd949176332f9134b286cdb35a46febb9f8cf)) - Luke Hsiao

### Features

- expose ttl as command line parameter - ([a2c43c7](https://github.com/lukehsiao/netlify-ddns-rs/commit/a2c43c7c7ee09d62a9b87fe6e31abe660e99cf40)) - Luke Hsiao

### Refactor

- **(netlify)** add_dns_record() now takes ownership - ([643dd4a](https://github.com/lukehsiao/netlify-ddns-rs/commit/643dd4ab548098e4c8b4d05d60c74a939f224cb2)) - Luke Hsiao
- switch from reqwest to ureq - ([c34ae26](https://github.com/lukehsiao/netlify-ddns-rs/commit/c34ae26aeac3a2ff5079a7fe2ea51f24f65d5170)) - Luke Hsiao

### Testing

- **(netlify)** fix test mock of getting DNS records - ([81d311d](https://github.com/lukehsiao/netlify-ddns-rs/commit/81d311dbef365fd8756f09b1c1df8f8840984d6e)) - Luke Hsiao
- **(netlify)** add tests for all netlify functions - ([b429ae6](https://github.com/lukehsiao/netlify-ddns-rs/commit/b429ae63a0bb9f3ef4886958127e855eaa8e96f6)) - Luke Hsiao

---
## [0.2.2](https://github.com/lukehsiao/netlify-ddns-rs/compare/v0.2.1..v0.2.2) - 2019-12-21

### Documentation

- **(CHANGELOG)** update for v0.2.2 - ([441befb](https://github.com/lukehsiao/netlify-ddns-rs/commit/441befbabb7e349da8bb8f32a4f91509c7dc8649)) - Luke Hsiao

### Refactor

- add info level logging - ([e1b00ea](https://github.com/lukehsiao/netlify-ddns-rs/commit/e1b00ea77af310e36b9bc12daf057c72a2cb8437)) - Luke Hsiao

### Testing

- **(lib)** fix reference to block_on - ([bc2b9cd](https://github.com/lukehsiao/netlify-ddns-rs/commit/bc2b9cd7a96e28d813a8a36661ba72849b299756)) - Luke Hsiao

---
## [0.2.1](https://github.com/lukehsiao/netlify-ddns-rs/compare/v0.2.0..v0.2.1) - 2019-11-08

### Documentation

- **(CHANGELOG)** update for v0.2.1 - ([1a6ca80](https://github.com/lukehsiao/netlify-ddns-rs/commit/1a6ca80e0b761e7814dbe2fbce56bd5cde21f258)) - Luke Hsiao
- update crate metadata and README for v0.2.1 - ([ac1dad0](https://github.com/lukehsiao/netlify-ddns-rs/commit/ac1dad01eb27d78457ee075739e1685634c65430)) - Luke Hsiao

### Refactor

- use async queries for external IP - ([a25d94d](https://github.com/lukehsiao/netlify-ddns-rs/commit/a25d94d5d169e51501d999219cec5fe75ceea811)) - Luke Hsiao

---
## [0.2.0](https://github.com/lukehsiao/netlify-ddns-rs/compare/v0.1.0..v0.2.0) - 2019-10-02

### Documentation

- **(README)** fix badge links - ([0192a99](https://github.com/lukehsiao/netlify-ddns-rs/commit/0192a99fffc04fd07845e736a7918c7423968bb2)) - Luke Hsiao

### Features

- **(lib)** add ipify.org as a fallback IP provider - ([faddb41](https://github.com/lukehsiao/netlify-ddns-rs/commit/faddb41c29cc95dea6ec972160a8f5bd916505e1)) - Luke Hsiao

### Refactor

- switch from clap to structopt - ([a0338ab](https://github.com/lukehsiao/netlify-ddns-rs/commit/a0338abd69233db5d990c8a58aede01c183024a3)) - Luke Hsiao

---
## [0.1.0] - 2019-08-28

### Bug Fixes

- **(netlify)** simplify netlify::DNSRecord - ([ac8410a](https://github.com/lukehsiao/netlify-ddns-rs/commit/ac8410a14c589f5bf74efb8e876db48795a28d0d)) - Luke Hsiao

### Documentation

- dual license under Apache-2.0 and MIT - ([4e4b8ee](https://github.com/lukehsiao/netlify-ddns-rs/commit/4e4b8eee227e2ac885e436229a39dbcdfc018308)) - Luke Hsiao
- add README and CHANGELOG - ([d863e81](https://github.com/lukehsiao/netlify-ddns-rs/commit/d863e81094d10b522ee9690c12da885478fc65be)) - Luke Hsiao

### Features

- **(lib)** add get_external_ip - ([15235eb](https://github.com/lukehsiao/netlify-ddns-rs/commit/15235ebc877acee603da9b025c483abe6741ff62)) - Luke Hsiao
- **(netlify)** add get_dns_records - ([663f0e0](https://github.com/lukehsiao/netlify-ddns-rs/commit/663f0e0289dedd57eea1d2d9be3f01686a9d4dc4)) - Luke Hsiao
- **(netlify)** add delete_dns_record and add_dns_record - ([38ea082](https://github.com/lukehsiao/netlify-ddns-rs/commit/38ea08293afa1601e6dcb6a65142f39130d58f59)) - Luke Hsiao
- add argument parsing - ([4334fe4](https://github.com/lukehsiao/netlify-ddns-rs/commit/4334fe4fdca72560a4c7075b15da8c93e25666ff)) - Luke Hsiao
- support loading token from $NETLIFY_TOKEN - ([ae96d70](https://github.com/lukehsiao/netlify-ddns-rs/commit/ae96d70811f25f831819d66e3b8aaa285a6b2794)) - Luke Hsiao

### Refactor

- **(main)** simplify error handling - ([47d2f94](https://github.com/lukehsiao/netlify-ddns-rs/commit/47d2f94117227ca8aa82acf799c6ec906d75a939)) - Luke Hsiao
- use & syntax rather than .as_str() - ([5433c27](https://github.com/lukehsiao/netlify-ddns-rs/commit/5433c27df67e244efcc364d6827d868ee5605e5c)) - Luke Hsiao

### Testing

- **(lib)** add mockito http mocks for get_external_ip - ([2957502](https://github.com/lukehsiao/netlify-ddns-rs/commit/2957502f6547ebc37a3cc5a534e81cc3b1a7e514)) - Luke Hsiao

