# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.1] - 2019-11-08
### Changed
- [@lukehsiao][lh]: Query external IP services asynchronously.

## [0.2.0] - 2019-10-02
### Changed
- [@lukehsiao][lh]: Switch argument parsing from clap to structopt. This
  switches the --ipv6 from being a flag to --ip-type=<IPV4|IPV6>.
- [@lukehsiao][lh]: Use ipify.org as a fallback IP provider for ident.me.

## [0.1.0] - 2019-08-28
Initial release on GitHub and crates.io.


[lh]: https://github.com/lukehsiao

[Unreleased]: https://github.com/lukehsiao/netlify-ddns-rs/compare/v0.2.1...master
[0.2.1]: https://github.com/lukehsiao/netlify-ddns-rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/lukehsiao/netlify-ddns-rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/lukehsiao/netlify-ddns-rs/releases/tag/v0.1.0
