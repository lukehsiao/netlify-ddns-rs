# netlify-ddns

[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/lukehsiao/netlify-ddns-rs/rust)](https://github.com/lukehsiao/netlify-ddns-rs/actions)
[![Crates.io](https://img.shields.io/crates/v/netlify-ddns)](https://crates.io/crates/netlify-ddns)
[![License](https://img.shields.io/crates/l/netlify-ddns)](https://github.com/lukehsiao/netlify-ddns-rs/blob/master/LICENSE-MIT)

netlify-ddns is a simple command line tool for creating a DNS record for
[Netlify's Managed DNS][netlify] service. It is meant to be run as a cron job
and queries third-parties (multiple, in case one is down) for your public IP,
then updates or adds a DNS record using the Netlify API.

## Installation

Install using [cargo][cargo]:

```
cargo install netlify-ddns
```

## Usage

```
netlify-ddns
A simple CLI tool for setting Netlify DNS records dynamically.

USAGE:
    netlify-ddns [OPTIONS] --domain <domain> --token <token>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --domain <domain>          The full domain for the DNS record
    -i, --ip-type <ip-type>        Whether an IPv6 'AAAA' record should be updated [default: ipv4]  [possible
                                   values: ipv4, ipv6]
    -s, --subdomain <subdomain>    The subdomain segment for the DNS record [default: www]
    -t, --token <token>            Your Netlify personal access token [env: NETLIFY_TOKEN=]
        --ttl <ttl>                The TTL value in seconds to set with the record [default: 3600]
```

## Example Cronjob

You could set a cronjob to update the `dev.example.com` hourly with a task like
the following.

```
0 * * * * netlify-ddns -d example.com -s dev --token=<your token>
```

## Related

* [oscartbeaumont/netlify-dynamic-dns] for a similar client written in [Go][go].
* [lytedev/netlify-ddns] for a simple shell script version.
* [skylerwlewis/netlify-ddns.sh] for another simple script version.

[cargo]: https://doc.rust-lang.org/cargo/getting-started/installation.html
[go]: https://golang.org/
[lytedev/netlify-ddns]: https://github.com/lytedev/netlify-ddns
[netlify]: https://www.netlify.com/docs/dns/
[oscartbeaumont/netlify-dynamic-dns]: https://github.com/oscartbeaumont/netlify-dynamic-dns
[skylerwlewis/netlify-ddns.sh]: https://gist.github.com/skylerwlewis/ba052db5fe26424255674931d43fc030
