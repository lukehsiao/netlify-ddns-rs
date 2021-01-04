#![forbid(unsafe_code)]
#![forbid(warnings)]

pub mod netlify;

use futures::future::FutureExt;
use futures::{executor, future};

use anyhow::{Context, Result};
use log::{debug, info};
use structopt::clap::arg_enum;
use structopt::clap::AppSettings;
use structopt::StructOpt;

use netlify::DNSRecord;

arg_enum! {
    #[derive(Debug)]
    pub enum IpType {
        IPV4,
        IPV6,
    }
}

#[derive(Debug, StructOpt)]
#[structopt(
    about,
    setting(AppSettings::ColoredHelp),
    setting(AppSettings::ColorAuto)
)]
pub struct Args {
    /// The full domain for the DNS record
    #[structopt(short, long)]
    pub domain: String,

    /// The subdomain segment for the DNS record
    #[structopt(short, long, default_value = "www")]
    pub subdomain: String,

    /// The TTL value in seconds to set with the record
    #[structopt(long, default_value = "3600")]
    pub ttl: u32,

    /// Whether an IPv6 "AAAA" or an IPv4 "A" record should be updated
    #[structopt(short, long, possible_values = &IpType::variants(), case_insensitive = true, default_value = "ipv4")]
    pub ip_type: IpType,

    /// Your Netlify personal access token
    #[structopt(short, long, env = "NETLIFY_TOKEN")]
    pub token: String,
}

async fn query_ident_me(ip_type: &IpType) -> Result<String> {
    #[cfg(test)]
    let resp = match ip_type {
        IpType::IPV4 => ureq::get(&mockito::server_url()).call()?,
        IpType::IPV6 => ureq::get(&mockito::server_url()).call()?,
    };
    #[cfg(not(test))]
    let resp = match ip_type {
        IpType::IPV4 => ureq::get("https://v4.ident.me/").call()?,
        IpType::IPV6 => ureq::get("https://v6.ident.me/").call()?,
    };

    let body = resp
        .into_string()
        .context("Failed to convert ident.me response into string.")?;
    Ok(body)
}

async fn query_ipify_org(ip_type: &IpType) -> Result<String> {
    #[cfg(test)]
    let resp = match ip_type {
        IpType::IPV4 => ureq::get(&mockito::server_url()).call()?,
        IpType::IPV6 => ureq::get(&mockito::server_url()).call()?,
    };
    #[cfg(not(test))]
    let resp = match ip_type {
        IpType::IPV4 => ureq::get("https://api.ipify.org/").call()?,
        IpType::IPV6 => ureq::get("https://api6.ipify.org/").call()?,
    };

    let body = resp
        .into_string()
        .context("Failed to convert ident.me response into string.")?;
    Ok(body)
}

// Get the host machine's external IP address by querying multiple services and
// taking the first response.
async fn get_external_ip(ip_type: &IpType) -> Result<String> {
    debug!("Querying third-party services for external IP...");

    let third_parties = vec![
        query_ident_me(ip_type).boxed(),
        query_ipify_org(ip_type).boxed(),
    ];

    // Select the first succesful future, or the last failure.
    let (ip, _) = future::select_ok(third_parties.into_iter())
        .await
        .context("All queries for external IP failed.")?;

    info!("Found External IP: {}", ip);
    Ok(ip)
}

fn get_conflicts(
    dns_records: Vec<DNSRecord>,
    args: &Args,
    rec: &DNSRecord,
) -> (Vec<DNSRecord>, Vec<DNSRecord>) {
    let target_hostname = format!(
        "{}{}{}",
        &args.subdomain,
        if &args.subdomain == "" { "" } else { "." },
        &args.domain
    );
    dns_records
        .into_iter()
        .filter(|r| match args.ip_type {
            IpType::IPV4 => r.dns_type == "A",
            IpType::IPV6 => r.dns_type == "AAAA",
        })
        .filter(|r| r.hostname == target_hostname)
        .partition(|r| r.hostname == target_hostname && r.value == rec.value)
}

pub fn run(args: Args) -> Result<()> {
    let ip = executor::block_on(get_external_ip(&args.ip_type))?;

    let rec = DNSRecord {
        hostname: args.subdomain.to_string(),
        dns_type: match args.ip_type {
            IpType::IPV4 => "A".to_string(),
            IpType::IPV6 => "AAAA".to_string(),
        },
        ttl: Some(args.ttl),
        value: ip,
        id: None,
    };

    // Update the DNS record if it exists, otherwise add.
    let dns_records = netlify::get_dns_records(&args.domain, &args.token)
        .context("Unable to fetch DNS records.")?;

    // Match on subdomain
    let (exact, conflicts) = get_conflicts(dns_records, &args, &rec);

    // Clear existing records for this subdomain, if any
    for r in conflicts {
        info!("Clearing conflicting DNS records for this subdomain.");
        netlify::delete_dns_record(&args.domain, &args.token, r)
            .context("Unable to delete DNS records.")?;
    }

    // Add new record
    if exact.is_empty() {
        info!("Adding the DNS record.");
        let rec = netlify::add_dns_record(&args.domain, &args.token, rec)
            .context("Unable to add the DNS record.")?;
        info!("{:#?}", rec);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use mockito::mock;

    #[test]
    fn test_get_external_ip() {
        let _m = mock("GET", "/")
            .with_status(200)
            .with_header("content-type", "text/plain")
            .with_body("104.132.34.103")
            .create();
        let ip = executor::block_on(get_external_ip(&IpType::IPV4)).unwrap();
        assert_eq!("104.132.34.103", &ip);

        let _m = mock("GET", "/")
            .with_status(200)
            .with_header("content-type", "text/plain")
            .with_body("2620:0:1003:fd00:95e9:369a:53cd:f035")
            .create();

        let ip = executor::block_on(get_external_ip(&IpType::IPV6)).unwrap();
        assert_eq!("2620:0:1003:fd00:95e9:369a:53cd:f035", &ip);
    }

    #[test]
    fn test_get_external_ip_404() {
        let _m = mock("GET", "/")
            .with_status(404)
            .with_header("content-type", "text/plain")
            .with_body("Not found")
            .create();

        if let Ok(_) = executor::block_on(get_external_ip(&IpType::IPV6)) {
            panic!("Should've gotten an error.");
        }
    }

    #[test]
    fn test_conflicts() {
        let dns_records = vec![
            // Basic subdomain, exact and non-exact
            DNSRecord {
                hostname: "sub.helloworld.com".to_string(),
                dns_type: "A".to_string(),
                ttl: Some(3600),
                id: Some("abc123".to_string()),
                value: "1.2.3.4".to_string(),
            },
            DNSRecord {
                hostname: "sub.helloworld.com".to_string(),
                dns_type: "A".to_string(),
                ttl: Some(3600),
                id: Some("abc123".to_string()),
                value: "9.9.9.9".to_string(),
            },
            // Glob subdomain, exact and non-exact
            DNSRecord {
                hostname: "*.sub.helloworld.com".to_string(),
                dns_type: "A".to_string(),
                ttl: Some(3600),
                id: Some("abc123".to_string()),
                value: "1.2.3.4".to_string(),
            },
            DNSRecord {
                hostname: "*.sub.helloworld.com".to_string(),
                dns_type: "A".to_string(),
                ttl: Some(3600),
                id: Some("abc123".to_string()),
                value: "9.9.9.9".to_string(),
            },
            // Empty subdomain, exact and non-exact
            DNSRecord {
                hostname: "helloworld.com".to_string(),
                dns_type: "A".to_string(),
                ttl: Some(3600),
                id: Some("abc123".to_string()),
                value: "1.2.3.4".to_string(),
            },
            DNSRecord {
                hostname: "helloworld.com".to_string(),
                dns_type: "A".to_string(),
                ttl: Some(3600),
                id: Some("abc123".to_string()),
                value: "9.9.9.9".to_string(),
            },
        ];

        let (glob_exact, glob_conflicts) = get_conflicts(
            dns_records.clone(),
            &Args {
                domain: "helloworld.com".to_string(),
                subdomain: "*.sub".to_string(),
                ttl: 3600,
                ip_type: IpType::IPV4,
                token: "123".to_string(),
            },
            &DNSRecord {
                hostname: "*.sub".to_string(),
                dns_type: "A".to_string(),
                ttl: Some(3600),
                id: None,
                value: "1.2.3.4".to_string(),
            },
        );
        assert_eq!(glob_conflicts.len(), 1);
        assert_eq!(glob_exact.len(), 1);

        let (sub_exact, sub_conflicts) = get_conflicts(
            dns_records.clone(),
            &Args {
                domain: "helloworld.com".to_string(),
                subdomain: "sub".to_string(),
                ttl: 3600,
                ip_type: IpType::IPV4,
                token: "123".to_string(),
            },
            &DNSRecord {
                hostname: "sub".to_string(),
                dns_type: "A".to_string(),
                ttl: Some(3600),
                id: None,
                value: "1.2.3.4".to_string(),
            },
        );
        assert_eq!(sub_conflicts.len(), 1);
        assert_eq!(sub_exact.len(), 1);

        let (empty_exact, empty_conflicts) = get_conflicts(
            dns_records.clone(),
            &Args {
                domain: "helloworld.com".to_string(),
                subdomain: "".to_string(),
                ttl: 3600,
                ip_type: IpType::IPV4,
                token: "123".to_string(),
            },
            &DNSRecord {
                hostname: "".to_string(),
                dns_type: "A".to_string(),
                ttl: Some(3600),
                id: None,
                value: "1.2.3.4".to_string(),
            },
        );
        assert_eq!(empty_conflicts.len(), 1);
        assert_eq!(empty_exact.len(), 1);
    }
}
