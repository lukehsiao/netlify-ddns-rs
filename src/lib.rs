extern crate failure;
extern crate reqwest;

use std::io::Read;

use failure::Error;

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
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_external_ip() {
        let ip = get_external_ip(IpType::IPV4).unwrap();
        assert_eq!("104.132.34.103", ip.as_str());

        let ip = get_external_ip(IpType::IPV6).unwrap();
        assert_eq!("2620:0:1003:fd00:95e9:369a:53cd:f035", ip.as_str());
    }
}
