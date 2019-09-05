#[macro_use]
extern crate failure;
extern crate reqwest;

pub mod netlify;

use std::io::Read;

use failure::Error;

use netlify::DNSRecord;

#[cfg(test)]
use mockito;

#[derive(Debug)]
pub struct Args {
    pub domain: String,
    pub subdomain: String,
    pub ip_type: IpType,
    pub token: String,
}

#[derive(PartialEq, Debug)]
pub enum IpType {
    IPV4,
    IPV6,
}

// Get the host machine's external IP address
fn get_external_ip(ip_type: &IpType) -> Result<String, Error> {
    let mut body = String::new();

    #[cfg(test)]
    let mut resp = match ip_type {
        IpType::IPV4 => reqwest::get(&mockito::server_url())?,
        IpType::IPV6 => reqwest::get(&mockito::server_url())?,
    };
    #[cfg(not(test))]
    let mut resp = match ip_type {
        IpType::IPV4 => reqwest::get("https://v4.ident.me/")?,
        IpType::IPV6 => reqwest::get("https://v6.ident.me/")?,
    };

    if resp.status().is_success() {
        resp.read_to_string(&mut body)?;
    } else {
        // Attempt a fallback service
        resp = match ip_type {
            IpType::IPV4 => reqwest::get("https://api.ipify.org/")?,
            IpType::IPV6 => reqwest::get("https://api6.ipify.org/")?,
        };

        if resp.status().is_success() {
            resp.read_to_string(&mut body)?;
        } else {
            bail!("Unable to get external IP.");
        }
    }
    Ok(body)
}

pub fn run(args: Args) -> Result<(), Error> {
    let ip = get_external_ip(&args.ip_type)?;

    let rec = DNSRecord {
        hostname: format!("{}.{}", &args.subdomain, &args.domain),
        dns_type: match args.ip_type {
            IpType::IPV4 => "A".to_string(),
            IpType::IPV6 => "AAAA".to_string(),
        },
        ttl: Some(3600),
        value: ip,
        id: None,
    };

    // Update the DNS record if it exists, otherwise add.
    let dns_records = netlify::get_dns_records(&args.domain, &args.token)?;

    // Match on subdomain
    // TODO: what if subdomain == ""?
    let (exact, conflicts): (Vec<DNSRecord>, Vec<DNSRecord>) = dns_records
        .into_iter()
        .filter(|r| match &args.ip_type {
            IpType::IPV4 => r.dns_type == "A",
            IpType::IPV6 => r.dns_type == "AAAA",
        })
        .filter(|r| {
            let v = r.hostname.split('.').collect::<Vec<&str>>();
            v.len() == 3 && v[0] == &args.subdomain
        })
        .partition(|r| r.hostname == rec.hostname && r.value == rec.value);

    // Clear existing records for this subdomain, if any
    for r in conflicts {
        netlify::delete_dns_record(&args.domain, &args.token, r)?;
    }

    // Add new record
    if exact.len() == 0 {
        netlify::add_dns_record(&args.domain, &args.token, &rec)?;
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
        let ip = get_external_ip(&IpType::IPV4).unwrap();
        assert_eq!("104.132.34.103", &ip);

        let _m = mock("GET", "/")
            .with_status(200)
            .with_header("content-type", "text/plain")
            .with_body("2620:0:1003:fd00:95e9:369a:53cd:f035")
            .create();

        let ip = get_external_ip(&IpType::IPV6).unwrap();
        assert_eq!("2620:0:1003:fd00:95e9:369a:53cd:f035", &ip);
    }
}
