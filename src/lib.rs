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

#[derive(Debug)]
pub enum IpType {
    IPV4,
    IPV6,
}

// Get the host machine's external IP address
fn get_external_ip(ip_type: IpType) -> Result<String, Error> {
    #[cfg(test)]
    let mut res = match ip_type {
        IpType::IPV4 => reqwest::get(&mockito::server_url())?,
        IpType::IPV6 => reqwest::get(&mockito::server_url())?,
    };
    #[cfg(not(test))]
    let mut res = match ip_type {
        IpType::IPV4 => reqwest::get("https://v4.ident.me/")?,
        IpType::IPV6 => reqwest::get("https://v6.ident.me/")?,
    };
    let mut body = String::new();
    res.read_to_string(&mut body)?;
    Ok(body)
}

pub fn run(args: Args) -> Result<(), Error> {
    let _ip = get_external_ip(args.ip_type)?;

    let dns_records = netlify::get_dns_records(&args.domain, &args.token)?;

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
        let ip = get_external_ip(IpType::IPV4).unwrap();
        assert_eq!("104.132.34.103", ip.as_str());

        let _m = mock("GET", "/")
            .with_status(200)
            .with_header("content-type", "text/plain")
            .with_body("2620:0:1003:fd00:95e9:369a:53cd:f035")
            .create();

        let ip = get_external_ip(IpType::IPV6).unwrap();
        assert_eq!("2620:0:1003:fd00:95e9:369a:53cd:f035", ip.as_str());
    }
}
