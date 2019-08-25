#[macro_use]
extern crate clap;
#[macro_use]
extern crate failure;

use clap::{App, AppSettings, Arg};
use failure::Error;

use netlify_ddns::{run, Args, IpType};

fn parse_args() -> Result<Args, Error> {
    // Setup CLI
    let matches = App::new(crate_name!())
        .author(crate_authors!("\n"))
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::with_name("token")
                .short("t")
                .long("token")
                .help("Your Netlify personal access token")
                .env("NETLIFY_TOKEN"),
        )
        .arg(
            Arg::with_name("subdomain")
                .short("s")
                .long("subdomain")
                .help("The subdomain segment for the DNS record")
                .default_value("www")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("domain")
                .short("d")
                .long("domain")
                .help("The full domain for the DNS record")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("ipv6")
                .long("ipv6")
                .help("Whether an IPv6 'AAAA' record should be updated"),
        )
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    // TODO: could switch to structopt or clap v3
    let domain = matches.value_of("domain").unwrap().to_string();
    let subdomain = matches.value_of("subdomain").unwrap().to_string();
    let ip_type = if matches.is_present("ipv6") {
        IpType::IPV6
    } else {
        IpType::IPV4
    };

    let token = match matches.value_of("token") {
        Some(token) => token.to_string(),
        None => bail!("No Netlify personal access token found."),
    };

    Ok(Args {
        domain,
        subdomain,
        ip_type,
        token,
    })
}

fn main() -> Result<(), Error> {
    let args = match parse_args() {
        Ok(args) => args,
        Err(e) => return Err(e),
    };

    return run(args);
}
