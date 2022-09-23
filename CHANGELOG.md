## 0.3.1 - 2022-09-23

### Build and Dependencies
- Update cargo dependencies
- (deps) Bump futures from 0.3.9 to 0.3.12
- (deps) Bump serde from 1.0.118 to 1.0.119
- (deps) Bump serde from 1.0.119 to 1.0.122
- (deps) Bump mockito from 0.28.0 to 0.29.0
- (deps) Bump serde from 1.0.122 to 1.0.123
- (deps) Bump log from 0.4.13 to 0.4.14
- (deps) Bump serde_json from 1.0.61 to 1.0.62
- (deps) Bump ureq from 2.0.1 to 2.0.2
- (deps) Bump thiserror from 1.0.23 to 1.0.24
- (deps) Bump serde_json from 1.0.62 to 1.0.64
- (deps) Bump futures from 0.3.12 to 0.3.13
- (deps) Bump serde from 1.0.123 to 1.0.124
- (deps) Bump anyhow from 1.0.38 to 1.0.39
- (deps) Bump mockito from 0.29.0 to 0.30.0
- Update dependencies and fix clippy lints
- (deps) Bump futures from 0.3.13 to 0.3.14
- (deps) Bump thiserror from 1.0.24 to 1.0.25
- (deps) Bump thiserror from 1.0.25 to 1.0.26
- (deps) Bump structopt from 0.3.21 to 0.3.22
- (deps) Bump serde_json from 1.0.64 to 1.0.66
- (deps) Bump serde_json from 1.0.66 to 1.0.67
- (deps) Bump thiserror from 1.0.26 to 1.0.28
- (deps) Bump structopt from 0.3.22 to 0.3.23
- (deps) Bump thiserror from 1.0.28 to 1.0.29
- (deps) Bump serde_json from 1.0.67 to 1.0.68
- (deps) Bump thiserror from 1.0.29 to 1.0.30
- (deps) Bump structopt from 0.3.23 to 0.3.25
- (deps) Bump serde_json from 1.0.68 to 1.0.69
- (deps) Bump serde_json from 1.0.69 to 1.0.70
- (deps) Bump serde_json from 1.0.70 to 1.0.71
- (deps) Bump serde_json from 1.0.71 to 1.0.72
- (deps) Bump serde_json from 1.0.72 to 1.0.73
- (deps) Bump serde_json from 1.0.73 to 1.0.74
- (deps) Bump serde_json from 1.0.74 to 1.0.75
- (deps) Bump structopt from 0.3.25 to 0.3.26
- (deps) Bump serde_json from 1.0.75 to 1.0.78
- (deps) Bump serde_json from 1.0.78 to 1.0.79
- (deps) Bump log from 0.4.14 to 0.4.16
- (deps) Bump actions/checkout from 2 to 3
- (deps) Bump serde_json from 1.0.79 to 1.0.80
- (deps) Bump thiserror from 1.0.30 to 1.0.31
- (deps) Bump mockito from 0.30.0 to 0.31.0
- (deps) Bump serde_json from 1.0.80 to 1.0.81
- (deps) Bump log from 0.4.16 to 0.4.17
- Add scripts and template for easier releases
- (deps) Bump serde_json from 1.0.81 to 1.0.82
- (deps) Bump serde_json from 1.0.82 to 1.0.85
- (deps) Bump thiserror from 1.0.31 to 1.0.32
- (deps) Bump thiserror from 1.0.32 to 1.0.34
- (deps) Bump thiserror from 1.0.34 to 1.0.35
- Update metadata, switch to rust 2021
- Switch from structopt to clap v3
- (deps) Update lockfile

### CI/CD
- Update github actions with dependabot

### Documentation
- (README) Update usage for latest clap output

### Miscellaneous Tasks
- Bump development version to 0.3.1-alpha.0
- Remove obsolete badges metadata

### Refactor
- Rename `DNSRecord` to `DnsRecord` and `IPVX` to `IpvX` to comply with
  [upper_case_acronyms](https://rust-lang.github.io/rust-clippy/master/index.html#upper_case_acronyms)
  lint.

See the commits here: [0.3.1]

[0.3.1]: https://github.com/lukehsiao/netlify-ddns-rs/compare/v0.3.0...v0.3.1

## [0.3.0] - 2021-01-04
### Removed
- [@lukehsiao][lh]: Removed the ExternalIPError type.

### Changed
- [@lukehsiao][lh]: Switch from `failure` to `thiserror` and `anyhow`.

## [0.2.4] - 2020-05-01
### Added
- [@ksmithbaylor][ksb]: Extend conflict detection to handle subdomain globs.

## [0.2.3] - 2020-02-04
### Added
- [@lukehsiao][lh]: Expose ttl setting as command line argument.

### Changed
- [@lukehsiao][lh]: Switch http library from reqwest to ureq.
- [@lukehsiao][lh]: `netlify::add_dns_record()` now takes ownership of the provided record.

### Fixed
- [@lukehsiao][lh]: Moved mockito to correctly be a dev-dependency.

## [0.2.2] - 2019-12-20
### Fixed
- [@lukehsiao][lh]: Fix clippy lints and update cargo dependencies.

## [0.2.1] - 2019-11-08
### Changed
- [@lukehsiao][lh]: Query external IP services asynchronously.

## [0.2.0] - 2019-10-02
### Changed
- [@lukehsiao][lh]: Switch argument parsing from clap to structopt. This switches the --ipv6 from
  being a flag to --ip-type=<IPV4|IPV6>.
- [@lukehsiao][lh]: Use ipify.org as a fallback IP provider for ident.me.

## [0.1.0] - 2019-08-28
Initial release on GitHub and crates.io.


[lh]: https://github.com/lukehsiao
[ksb]: https://github.com/ksmithbaylor

[Unreleased]: https://github.com/lukehsiao/netlify-ddns-rs/compare/v0.3.0...master
[0.3.0]: https://github.com/lukehsiao/netlify-ddns-rs/compare/v0.2.4...v0.3.0
[0.2.4]: https://github.com/lukehsiao/netlify-ddns-rs/compare/v0.2.3...v0.2.4
[0.2.3]: https://github.com/lukehsiao/netlify-ddns-rs/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/lukehsiao/netlify-ddns-rs/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/lukehsiao/netlify-ddns-rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/lukehsiao/netlify-ddns-rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/lukehsiao/netlify-ddns-rs/releases/tag/v0.1.0
