#[macro_use]
extern crate clap;

use std::process;

use clap::{App, AppSettings, Arg};

fn parse_args() -> netlify_ddns::Args {
    // Setup CLI
    let matches = App::new(crate_name!())
        .author(crate_authors!("\n"))
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::with_name("token")
                .short("t")
                .long("token")
                .value_name("TOKEN")
                .help("Your Netlify personal access token")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("subdomain")
                .short("s")
                .long("subdomain")
                .help("The subdomain segment for the DNS record")
                .default_value("www")
                .value_name("SUBDOMAIN")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("domain")
                .short("d")
                .long("domain")
                .help("The full domain for the DNS record")
                .required(true)
                .value_name("DOMAIN")
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
    let ipv6 = matches.is_present("ipv6");
    let token = matches.value_of("token").unwrap().to_string();

    netlify_ddns::Args {
        domain,
        subdomain,
        ipv6,
        token,
    }
}

fn main() -> Result<(), ()> {
    let args = parse_args();

    if let Err(e) = netlify_ddns::run(args) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }

    Ok(())
}
